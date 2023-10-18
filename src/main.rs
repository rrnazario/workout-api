#![feature(async_fn_in_trait)]
#[macro_use]
extern crate rocket;

use dotenvy::dotenv;
use middleware::authorization::Authorization;
use middleware::cors::Cors;
use rocket::serde::json::{serde_json::json, Value};
use rocket::Request;

use crate::features::exercises::routes::get_all_exercises;

mod helpers;
mod middleware;
mod models;
mod features;
mod schema;
mod infrastructure;

#[catch(500)]
async fn server_error<'a>(req: &'a Request<'_>) -> Value {
    let message = format!("URL: {}", req.uri().to_string());

    println!("{}", message);

    json!({
        "message": "A server error occurred"
    })
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "message": "resource not found"
    })
}

#[options("/<_..>")]
fn all_options() {}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(Cors)
        .attach(Authorization)

        .mount("/", routes![all_options])


        .mount("/", routes![get_all_exercises])

        .register("/", catchers![not_found, server_error])
}
