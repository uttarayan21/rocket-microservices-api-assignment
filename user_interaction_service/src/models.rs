use super::schema::user_interactions;
use diesel::sql_types::{BigInt, Integer};
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;

#[derive(Queryable, AsChangeset, JsonSchema, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "user_interactions"]
pub struct UserInteraction {
    pub id: i32,
    pub user_id: i32,
    pub content_id: i32,
    pub user_read: bool,
    pub user_like: bool,
}

impl UserInteraction {
    pub fn liked(&self) -> Self {
        Self {
            id: self.id,
            user_id: self.user_id,
            content_id: self.content_id,
            user_read: self.user_read,
            user_like: true,
        }
    }

    pub fn unliked(&self) -> Self {
        Self {
            id: self.id,
            user_id: self.user_id,
            content_id: self.content_id,
            user_read: self.user_read,
            user_like: false,
        }
    }

    pub fn read(&self) -> Self {
        Self {
            id: self.id,
            user_id: self.user_id,
            content_id: self.content_id,
            user_read: true,
            user_like: self.user_like,
        }
    }

    // Probably not needed
    pub fn unread(&self) -> Self {
        Self {
            id: self.id,
            user_id: self.user_id,
            content_id: self.content_id,
            user_read: false,
            user_like: self.user_like,
        }
    }
}

#[derive(Debug, Insertable, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[table_name = "user_interactions"]
pub struct InsertableUserInteraction {
    pub user_id: i32,
    pub content_id: i32,
    pub user_read: bool,
    pub user_like: bool,
}

#[derive(Debug, Serialize, Queryable, JsonSchema, QueryableByName)]
#[serde(crate = "rocket::serde")]
pub struct ContentFrequency {
    #[sql_type = "Integer"]
    pub content_id: i32,
    #[sql_type = "BigInt"]
    pub frequency: i64,
}
