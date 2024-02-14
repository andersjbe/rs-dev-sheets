use diesel::{self, deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::schema::*;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    #[typeshare(skip)]
    #[serde(skip_serializing)]
    pub pw_hash: String,
    pub profile_image_url: String,
    pub created_at: chrono::NaiveDateTime,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub profile_image_url: Option<String>,
}
