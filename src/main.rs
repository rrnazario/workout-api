#![allow(dead_code, unused_variables)]
#![feature(async_fn_in_trait)]
#[macro_use]
extern crate rocket;

use dotenvy::dotenv;
use middleware::authorization::Authorization;
use middleware::cors::Cors;

use crate::features::exercises::routes::get_all_exercises;
use crate::helpers::routes_helper::*;

mod features;
mod helpers;
mod infrastructure;
mod middleware;
mod models;
mod schema;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(Cors)
        .attach(Authorization)
        .mount("/", routes![all_options, fail_unauthorized])
        .mount("/", routes![get_all_exercises])
        .register("/", catchers![not_found, server_error])
}
