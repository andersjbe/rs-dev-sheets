use diesel::{self, deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::schema::users;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
}

impl NewUser {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
