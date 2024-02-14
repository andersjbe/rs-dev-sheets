use actix_web::web;

use crate::handlers::users;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(web::resource("").route(web::post().to(users::add_user)))
            .service(web::resource("/login").route(web::post().to(users::login)))
            .service(web::resource("/me").route(web::get().to(users::get_me)))
            .service(web::resource("/logout").route(web::delete().to(users::logout)))
            //? Wildcard / Catch all routes need to go at the bottom of a scope
            .service(web::resource("/{user_id}").route(web::get().to(users::get_user))),
    );
}
