use std::future::{ready, Ready};

use actix_identity::Identity;
use actix_web::{
    dev::Payload, error, web, Error, FromRequest, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use rs_dev_sheets::DbPool;
use uuid::Uuid;

use crate::{
    actions,
    errors::ServiceError,
    models::{NewUser, SlimUser, User, UserLogin},
};

pub type LoggedUser = SlimUser;
impl FromRequest for User {
    type Error = Error;
    type Future = Ready<Result<User, Error>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, pl).into_inner() {
            if let Ok(user_json) = identity.id() {
                match serde_json::from_str::<User>(&user_json) {
                    Ok(user) => return ready(Ok(user)),
                    Err(err) => {
                        println!("{err}");
                        return ready(Err(ServiceError::BadRequest(err.to_string()).into()));
                    }
                }
            }
        }

        ready(Err(ServiceError::Unauthorized.into()))
    }
}

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

pub async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<NewUser>,
) -> actix_web::Result<impl Responder> {
    let user = web::block(move || {
        let mut conn = pool.get()?;

        actions::insert_new_user(&mut conn, form.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(user))
}

pub async fn login(
    req: HttpRequest,
    auth_data: web::Json<UserLogin>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = web::block(move || {
        let mut conn = pool.get().unwrap();

        actions::verify_user(&mut conn, auth_data.into_inner())
    })
    .await??;

    let user_string = serde_json::to_string(&user).unwrap();
    Identity::login(&req.extensions(), user_string).unwrap();

    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_me(logged_user: User) -> HttpResponse {
    HttpResponse::Ok().json(logged_user)
}

pub async fn logout(id: Identity) -> HttpResponse {
    id.logout();
    HttpResponse::NoContent().finish()
}
