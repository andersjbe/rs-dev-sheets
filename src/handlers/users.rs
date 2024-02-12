use actix_web::{error, web, HttpResponse, Responder};
use rs_dev_sheets::DbPool;
use uuid::Uuid;

use crate::{actions, models::NewUser};

// #[get("/user/{user_id}")]
pub async fn get_user(
    pool: web::Data<DbPool>,
    user_uid: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let user_uid = user_uid.into_inner();

    let user = web::block(move || {
        let mut conn = pool.get()?;

        actions::find_user_by_uid(&mut conn, user_uid)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body(format!("No user found with UID: {user_uid}")),
    })
}

// #[post("/user")]
pub async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<NewUser>,
) -> actix_web::Result<impl Responder> {
    let user = web::block(move || {
        let mut conn = pool.get()?;

        actions::insert_new_user(&mut conn, &form.name)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(user))
}
