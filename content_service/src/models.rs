use super::schema::contents;
use chrono::DateTime;
use rocket::serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Queryable, QueryableByName, JsonSchema, AsChangeset, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "contents"]
pub struct Content {
    pub id: i32,
    pub title: String,
    pub story: String,
    pub published: DateTime<chrono::Utc>,
    pub user_id: i32,
}

#[derive(Debug, Insertable, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[table_name = "contents"]
pub struct InsertableContent {
    pub title: String,
    pub story: String,
    #[serde(deserialize_with = "chrono::serde::ts_seconds::deserialize")]
    pub published: DateTime<chrono::Utc>,
    pub user_id: i32,
}
