use super::connection;
use super::crud;
use super::models::{InsertableUser, User};
use rocket::serde::json::Json;
use rocket_okapi::openapi;

pub fn to_rocket_response(err: diesel::result::Error) -> rocket::http::Status {
    match err {
        diesel::result::Error::NotFound => rocket::http::Status::NotFound,
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        ) => rocket::http::Status::Conflict,
        _ => {
            println!("{:?}", err);
            rocket::http::Status::InternalServerError
        }
    }
}

/// # Get all users
#[openapi]
#[get("/")]
pub fn all() -> Result<Json<Vec<User>>, rocket::http::Status> {
    let c = connection::connect();
    crud::all(&c).map(Json).map_err(to_rocket_response)
}

/// # Get user by id
#[openapi]
#[get("/<id>")]
pub fn get(id: i32) -> Result<Json<User>, rocket::http::Status> {
    let c = connection::connect();
    crud::get(id, &c).map(Json).map_err(to_rocket_response)
}

/// # Insert user into table
#[openapi]
#[post("/", format = "application/json", data = "<user>")]
pub fn insert(user: Json<InsertableUser>) -> Result<Json<User>, rocket::http::Status> {
    let c = connection::connect();
    crud::insert(user.into_inner(), &c)
        .map(Json)
        .map_err(to_rocket_response)
}

/// # Update user by id
#[openapi]
#[put("/<id>", format = "application/json", data = "<user>")]
pub fn update(id: i32, user: Json<User>) -> Result<Json<User>, rocket::http::Status> {
    let c = connection::connect();
    crud::update(id, user.into_inner(), &c)
        .map(Json)
        .map_err(to_rocket_response)
}

/// # Delete user by id
#[openapi]
#[delete("/<id>")]
pub fn delete(id: i32) -> Result<Json<usize>, rocket::http::Status> {
    let c = connection::connect();
    crud::delete(id, &c).map(Json).map_err(to_rocket_response)
}
