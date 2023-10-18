use rocket::Responder;
use rocket::serde::{Serialize, Deserialize};

#[derive(Responder, Debug, Serialize, Deserialize)]
pub enum NetworkResponse {
    #[response(status = 201)]
    Created(String),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 401)]
    Unauthorized(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 409)]
    Conflict(String),
}

#[derive(Serialize)]
pub struct ResponseBody {
    pub message: String
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub message: ResponseBody,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub role: String,
    pub iss: String,
    pub aud: String,
    pub exp: usize
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims
}