use diesel::{r2d2, SqliteConnection};

pub mod actions;
pub mod models;
pub mod schema;

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;
