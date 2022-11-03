use crate::handlers::*;
use actix_web::web;

pub const IGNORE_ROUTES: [&str; 3] = ["/api/ping", "/api/login", "/api/register"];

pub fn config_services(cfg: &mut web::ServiceConfig) {
    log::info!("Configuring routes...");
    cfg.service(
        web::scope("/api")
            .service(
                web::resource("/ping")
                    .route(web::get().to(ping_handler::ping)),
            )
            .service(
                web::resource("/invitations")
                    .route(web::post().to(invitation_handler::post_invitation)),
            )
            .service(
                web::resource("/register/{invitation_id}")
                    .route(web::post().to(register_handler::register_user)),
            )
            .service(
                web::resource("/auth")
                    .route(web::get().to(auth_handler::get_me)),
            )
            .route("/login", web::post().to(auth_handler::login))
            .route("/logout", web::delete().to(auth_handler::logout))
    );
}
