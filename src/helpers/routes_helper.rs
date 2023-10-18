use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::Request;

use crate::models::dtos::NetworkResponse;

#[catch(500)]
pub fn server_error<'a>(req: &'a Request<'_>) -> Value {
    let message = format!("URL: {}", req.uri().to_string());

    println!("{}", message);

    json!({
        "message": "A server error occurred"
    })
}

#[catch(404)]
pub fn not_found() -> Value {
    json!({
        "message": "resource not found"
    })
}

#[options("/<_..>")]
pub fn all_options() {}

#[put("/fail/unauthorized")]
pub fn fail_unauthorized() -> Result<Value, NetworkResponse> {
    Err(NetworkResponse::Unauthorized("Unauthorized".to_owned()))
}