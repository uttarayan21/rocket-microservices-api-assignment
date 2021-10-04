use super::{
    connection, crud,
    models::{InsertableUserInteraction, UserInteraction},
};
use rocket_okapi::openapi;

// use rocket::data::Capped;
// use rocket::fs::TempFile;
use rocket::serde::json::Json;

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


/// # Get the number of likes of a certain content
#[openapi]
#[get("/get_likes?<content_id>")]
pub fn get_likes(content_id: usize) -> Result<Json<i64>, rocket::http::Status> {
    use std::convert::TryInto;
    let c = connection::connect();
    crud::get_content_likes(content_id.try_into().expect("Value too big"), &c)
        .map(Json)
        .map_err(to_rocket_response)
}

/// # Get the number of reads of a certain content
#[openapi]
#[get("/get_reads?<content_id>")]
pub fn get_reads(content_id: usize) -> Result<Json<i64>, rocket::http::Status> {
    use std::convert::TryInto;
    let c = connection::connect();
    crud::get_content_reads(content_id.try_into().expect("Value too big"), &c)
        .map(Json)
        .map_err(to_rocket_response)
}

/// # Get the content_id's sorted by user_interactions ( likes or reads )
#[openapi]
#[get("/get_top?<limit>&<sort_by>")]
pub fn get_top(
    limit: Option<usize>,
    sort_by: &str,
) -> Result<Json<Vec<super::models::ContentFrequency>>, rocket::http::Status> {
    let c = connection::connect();
    match sort_by.to_lowercase().as_str() {
        "likes" => crud::get_content_top_likes(limit.unwrap_or(10), &c)
            .map(Json)
            .map_err(to_rocket_response),
        "reads" => crud::get_content_top_reads(limit.unwrap_or(10), &c)
            .map(Json)
            .map_err(to_rocket_response),
        _ => Err(rocket::http::Status::BadRequest),
    }
}

/// # Insert data to the user_interaction_service
#[openapi]
#[post("/", format = "application/json", data = "<user_interaction>")]
pub fn insert(
    user_interaction: Json<InsertableUserInteraction>,
) -> Result<Json<UserInteraction>, rocket::http::Status> {
    let c = connection::connect();
    crud::insert(user_interaction.into_inner(), &c)
        .map(Json)
        .map_err(to_rocket_response)
}

/// # Like/unlike/read a content by a user
#[openapi]
#[post("/<content_id>/<user_id>?<action>")]
pub async fn action(
    content_id: i32,
    user_id: i32,
    action: &str,
) -> Result<Json<UserInteraction>, rocket::http::Status> {
    let c = connection::connect();

    // Need to validate if the user exists first.
    match validate_user(user_id).await {
        ValidationStatus::NotFound => return Err(rocket::http::Status::NoContent),
        ValidationStatus::Found => println!("user_id was validated"),
        _ => return Err(rocket::http::Status::Unauthorized),
    }

    match action.to_lowercase().as_str() {
        "read" => crud::read(content_id, user_id, &c)
            .map(Json)
            .map_err(to_rocket_response),
        "like" => crud::read_and_like(content_id, user_id, &c)
            .map(Json)
            .map_err(to_rocket_response),
        "unlike" => crud::unlike(content_id, user_id, &c)
            .map(Json)
            .map_err(to_rocket_response),

        _ => Err(rocket::http::Status::BadRequest),
    }
}

/// # Update user_interaction by id
///
/// (Probably not needed for this table but provided just in case)
#[openapi]
#[put("/?<id>", format = "application/json", data = "<user_interaction>")]
pub fn update(
    id: i32,
    user_interaction: Json<UserInteraction>,
) -> Result<Json<UserInteraction>, rocket::http::Status> {
    let c = connection::connect();
    crud::update(id, user_interaction.into_inner(), &c)
        .map(Json)
        .map_err(to_rocket_response)
}

/// # Delete user_interaction by id
#[openapi]
#[delete("/?<id>")]
pub fn delete(id: i32) -> Result<Json<usize>, rocket::http::Status> {
    let c = connection::connect();
    crud::delete(id, &c).map(Json).map_err(to_rocket_response)
}

pub enum ValidationStatus {
    NotFound,
    Found,
    Error,
}

pub async fn validate_user(user_id: i32) -> ValidationStatus {
    use hyper::{Client, Uri};
    use std::env;

    let mut user_service_url =
        env::var("USER_SERVICE_URL").expect("USER_SERVICE_URL must be set for validating users");
    user_service_url.push_str(&user_id.to_string());
    println!("{}", user_service_url);
    let client = Client::new();
    let url: Uri = user_service_url
        .parse()
        .expect("Unable to parse user_service url");

    match client.get(url).await {
        Ok(res) => {
            println!("Response: {}", res.status());
            if res.status() == 200 {
                ValidationStatus::Found
            } else {
                ValidationStatus::NotFound
            }
        }
        Err(err) => {
            println!("Error: {}", err);
            ValidationStatus::Error
        }
    }
}
