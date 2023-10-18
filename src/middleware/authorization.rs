use crate::helpers::jwt_helper::JwtHelper;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{uri::Origin, Method},
    Data, Request,
};

pub struct Authorization;

impl Authorization {
    fn get_whitelist<'a>(&self) -> Vec<(&'a str, &'a Method)> {
        vec![("/login", &Method::Post)]
    }
}

#[rocket::async_trait]
impl Fairing for Authorization {
    fn info(&self) -> Info {
        Info {
            name: "Authorization-info",
            kind: Kind::Request,
        }
    }
    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        let whitelisted = self
            .get_whitelist()
            .into_iter()
            .any(|(path, method)| request.uri().path().eq(path) && request.method().eq(method));

        if whitelisted {
            return;
        }

        if let Some(token) = request.headers().get_one("authorization") {
            let result = JwtHelper::decode_jwt(token.to_owned());

            if result.is_err() {
                let u = Origin::parse("/fail/unauthorized").unwrap();
                request.set_uri(u);
                request.set_method(Method::Put);

                return;
            }
        }
    }
}
