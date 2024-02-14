use diesel::prelude::*;
use pwhash::bcrypt::{hash, verify};
use uuid::Uuid;

use crate::{
    errors::ServiceError,
    models::{NewUser, User, UserLogin},
};

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

pub fn verify_user(
    conn: &mut SqliteConnection,
    auth_data: UserLogin,
) -> Result<User, ServiceError> {
    use crate::schema::users::dsl::{email, users};

    let mut items = users.filter(email.eq(auth_data.email)).load::<User>(conn)?;

    if let Some(user) = items.pop() {
        let matching = verify(&auth_data.password, &user.pw_hash);
        if matching {
            return Ok(user.into());
        }
    }

    Err(ServiceError::Unauthorized)
}

pub fn insert_new_user(conn: &mut SqliteConnection, user_data: NewUser) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    let default_profile_pic: String = "https://upload.wikimedia.org/wikipedia/commons/thumb/2/2c/Default_pfp.svg/2048px-Default_pfp.svg.png".to_string();
    let pfp_url = match user_data.profile_image_url {
        Some(url) => url,
        None => default_profile_pic,
    };

    let hash = hash(&user_data.password)?;

    let new_user = User {
        id: Uuid::new_v4().to_string(),
        email: user_data.email,
        username: user_data.username,
        profile_image_url: pfp_url,
        pw_hash: hash,
        created_at: chrono::offset::Utc::now().naive_utc(),
    };

    let _ = diesel::insert_into(users).values(&new_user).execute(conn);

    Ok(new_user)
}
