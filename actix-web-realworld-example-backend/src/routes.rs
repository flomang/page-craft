use crate::handlers::*;
use actix_web::web;

pub const IGNORE_ROUTES: [&str; 4] = ["/api/ping", "/api/login", "/api/register", "/api/users"];

pub fn config_services(cfg: &mut web::ServiceConfig) {
    log::info!("Configuring routes...");
    cfg.service(
        web::scope("/api")
            .service(web::resource("/ping").route(web::get().to(ping::ping)))
            // .service(
            //     web::resource("/invitations")
            //         .route(web::post().to(invitation::post_invitation)),
            // )
            // .service(
            //     web::resource("/register/{invitation_id}")
            //         .route(web::post().to(register::register_user)),
            // )
            // .service(web::resource("/auth").route(web::get().to(auth::get_me)))
            .route("/login", web::post().to(users::login))
            .route("/logout", web::post().to(users::logout))

            // User routes ↓
            .service(web::resource("/users").route(web::post().to(users::post_user)))
            // .service(web::resource("users/login").route(web::post().to_async(users::login)))
            // .service(
            //     web::resource("user")
            //         .route(web::get().to_async(users::get_current))
            //         .route(web::put().to_async(users::update)),
            // )
            // // Profile routes ↓
            // .service(web::resource("profiles/{username}").route(web::get().to_async(profiles::get)))
            // .service(
            //     web::resource("profiles/{username}/follow")
            //         .route(web::post().to_async(profiles::follow))
            //         .route(web::delete().to_async(profiles::unfollow)),
            // )
            // // Article routes ↓
            // .service(
            //     web::resource("articles")
            //         .route(web::get().to_async(articles::list))
            //         .route(web::post().to_async(articles::create)),
            // )
            // .service(web::resource("articles/feed").route(web::get().to_async(articles::feed)))
            // .service(
            //     web::resource("articles/{slug}")
            //         .route(web::get().to_async(articles::get))
            //         .route(web::put().to_async(articles::update))
            //         .route(web::delete().to_async(articles::delete)),
            // )
            // .service(
            //     web::resource("articles/{slug}/favorite")
            //         .route(web::post().to_async(articles::favorite))
            //         .route(web::delete().to_async(articles::unfavorite)),
            // )
            // .service(
            //     web::resource("articles/{slug}/comments")
            //         .route(web::get().to_async(articles::comments::list))
            //         .route(web::post().to_async(articles::comments::add)),
            // )
            // .service(
            //     web::resource("articles/{slug}/comments/{comment_id}")
            //         .route(web::delete().to_async(articles::comments::delete)),
            // )
            // // Tags routes ↓
            // .service(web::resource("tags").route(web::get().to_async(tags::get))),
    );
}
