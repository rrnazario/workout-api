use std::env;

use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    Request, Response,
};

/// `Cors` configuration
pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "CORS-info",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        let allowed_origins = env::var("CORS_ORIGIN")
        .expect("CORS_ORIGIN not informed");
        
        let allowed_origins: Vec<&str> = allowed_origins
        .split(",")
        .collect();

        for origin in allowed_origins {
            response.set_header(Header::new(
                "Access-Control-Allow-Origin",
                origin.trim().to_string(),
            ));
        }

        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, GET, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}