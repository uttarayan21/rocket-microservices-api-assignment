#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod catcher;
mod connection;
mod crud;
mod models;
mod routes;
mod schema;

use catcher::{default, internal_error, not_found};
use diesel_migrations::embed_migrations;
use routes::*;

use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::openapi_get_routes;

embed_migrations!();

#[launch]
fn rocket() -> _ {
    let connection = connection::connect();
    embedded_migrations::run(&connection).expect("Failed to run diesel migrations");

    rocket::build()
        .mount("/", openapi_get_routes![all, get, insert, update, delete])
        .mount("/swagger",make_swagger_ui(&get_docs()))
        .register("/", catchers![internal_error, not_found, default])
}

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/openapi.json".to_string(),
        ..Default::default()
    }
}
