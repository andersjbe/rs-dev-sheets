use diesel::prelude::*;
use uuid::Uuid;

use crate::models::User;

/// An error that can sent and shared across threads
type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_user_by_uid(conn: &mut SqliteConnection, uid: Uuid) -> Result<Option<User>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(uid.to_string()))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

pub fn insert_new_user(conn: &mut SqliteConnection, nm: &str) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    let new_user = User {
        id: Uuid::new_v4().to_string(),
        name: nm.to_owned(),
    };

    let _ = diesel::insert_into(users).values(&new_user).execute(conn);

    Ok(new_user)
}
