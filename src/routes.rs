use actix_web::web;

use crate::handlers::users;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(web::resource("").route(web::post().to(users::add_user)))
            .service(web::resource("/{user_id}").route(web::get().to(users::get_user))),
    );
}
