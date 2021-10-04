use super::{
    connection, crud,
    models::{Content, InsertableContent},
};

use rocket::data::Capped;
use rocket::fs::TempFile;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
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


/// # Get all contents
///
/// Route to get all contents in a json array
#[openapi]
#[get("/")]
pub fn all() -> Result<Json<Vec<Content>>, rocket::http::Status> {
    let c = connection::connect();
    crud::all(&c).map(Json).map_err(to_rocket_response)
}

/// # Get certain content by id
///
/// Route to get content by id
#[openapi]
#[get("/<id>")]
pub fn get(id: i32) -> Result<Json<Content>, rocket::http::Status> {
    let c = connection::connect();
    crud::get(id, &c).map(Json).map_err(to_rocket_response)
}


/// # Insert content to table
///
/// Route to insert content
#[openapi]
#[post("/", format = "application/json", data = "<content>")]
pub fn insert(content: Json<InsertableContent>) -> Result<Json<Content>, rocket::http::Status> {
    let c = connection::connect();
    crud::insert(content.into_inner(), &c)
        .map(Json)
        .map_err(to_rocket_response)
}


/// # Update Content by id
///
/// Route to update content by ID
#[openapi]
#[put("/<id>", format = "application/json", data = "<content>")]
pub fn update(id: i32, content: Json<Content>) -> Result<Json<Content>, rocket::http::Status> {
    let c = connection::connect();
    crud::update(id, content.into_inner(), &c)
        .map(Json)
        .map_err(to_rocket_response)
}


/// # Delete content by id
///
/// Route to delete content by id
#[openapi]
#[delete("/<id>")]
pub fn delete(id: i32) -> Result<Json<usize>, rocket::http::Status> {
    let c = connection::connect();
    crud::delete(id, &c).map(Json).map_err(to_rocket_response)
}

// This is a *raw* file upload, _not_ a multipart upload!
/// # Ingest CSV data
///
/// Route to ingest CSV data
#[openapi]
#[post("/ingest", data = "<file>")]
pub async fn ingest(mut file: Capped<TempFile<'_>>) -> Result<Json<usize>, rocket::http::Status> {
    file.persist_to("/tmp/story.csv")
        .await
        .expect("Failed to write file to disk");
    println!(
        "{} bytes at {}",
        file.n.written,
        file.path().expect("Failed to  get file path").display()
    );

    let mut rdr = csv::Reader::from_reader(
        std::fs::File::open("/tmp/story.csv").expect("Failed to open the csv file"),
    );

    // let contents = rdr
    //     .deserialize()
    //     .map(|record| {
    //         let result: InsertableContent = record.expect("Invalid csv");
    //         result;
    //     })
    //     .collect();

    let mut contents: Vec<InsertableContent> = vec![];
    for result in rdr.deserialize() {
        let record: InsertableContent = result.expect("Invalid csv");
        contents.push(record);
    }

    let c = connection::connect();
    crud::ingest(contents, &c)
        .map(Json)
        .map_err(to_rocket_response)
}

/// # Get new content sorted by creation date
/// 
/// Route to get new content
#[openapi]
#[get("/new?<limit>")]
pub fn new(limit: Option<usize>) -> Result<Json<Vec<Content>>, rocket::http::Status> {
    use std::convert::TryInto;
    let c = connection::connect();
    match limit {
        Some(_limit) => crud::new(_limit.try_into().expect("Too large or too small limit"), &c)
            .map(Json)
            .map_err(to_rocket_response),
        None => crud::new(10, &c).map(Json).map_err(to_rocket_response),
    }
}

/// # Get top content sorted by user interactions ( likes and reads )
///
/// Route to get top content by user_interactions
#[openapi]
#[get("/top?<limit>&<sort_by>")]
pub async fn top(
    limit: Option<usize>,
    sort_by: Option<&str>,
) -> Result<Json<Vec<Content>>, rocket::http::Status> {
    let c = connection::connect();

    let s = match sort_by {
        Some(sort_by) => match sort_by {
            "likes" => SortBy::Like,
            "reads" => SortBy::Read,
            _ => return Err(rocket::http::Status::BadRequest),
        },
        None => SortBy::Read,
    };
    let top_content_ids = get_top_content(limit.unwrap_or(10) as i32, s).await?;
    println!("{:?}", top_content_ids);

    crud::get_list(top_content_ids, &c)
        .map(Json)
        .map_err(to_rocket_response)
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ContentFrequency {
    content_id: i32,
    #[serde(rename(deserialize = "frequency"))]
    _frequency: i64,
}

#[derive(Debug)]
pub enum SortBy {
    Like,
    Read,
}

pub async fn get_top_content(
    limit: i32,
    sort_by: SortBy,
) -> Result<Vec<i32>, rocket::http::Status> {
    use hyper::{body::Buf, Client, Uri};
    use std::env;

    let mut user_interaction_service_url = env::var("USER_INTERACTION_SERVICE_URL")
        .expect("USER_INTERACTION_SERVICE_URL must be set for validating users");

    match sort_by {
        SortBy::Like => user_interaction_service_url
            .push_str(&format!("/get_top?limit={}&sort_by=likes", limit)),
        SortBy::Read => user_interaction_service_url
            .push_str(&format!("/get_top?limit={}&sort_by=reads", limit)),
    }

    let client = Client::new();
    let url: Uri = user_interaction_service_url
        .parse()
        .expect("Unable to parse user_service url");

    match client.get(url).await {
        Ok(res) => {
            println!("Response: {}", res.status());
            if res.status() == 200 {
                let body = hyper::body::aggregate(res).await.unwrap();

                let contents: Vec<ContentFrequency> =
                    serde_json::from_reader(body.reader()).unwrap();
                if contents.is_empty() {
                    Err(rocket::http::Status::NoContent)
                } else {
                    Ok(contents.into_iter().map(|c| c.content_id).collect())
                }
            } else {
                Err(rocket::http::Status::NotFound)
            }
        }
        Err(err) => {
            println!("Error: {}", err);
            Err(rocket::http::Status::ServiceUnavailable)
        }
    }
}
