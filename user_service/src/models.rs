use super::schema::users;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;

#[derive(Queryable, AsChangeset, JsonSchema, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email_id: String,
    pub phone_number: String,
}

#[derive(Insertable, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct InsertableUser {
    pub first_name: String,
    pub last_name: String,
    pub email_id: String,
    pub phone_number: String,
}

impl InsertableUser {
    pub fn from_user(user: User) -> InsertableUser {
        InsertableUser {
            first_name: user.first_name,
            last_name: user.last_name,
            email_id: user.email_id,
            phone_number: user.phone_number,
        }
    }
}
