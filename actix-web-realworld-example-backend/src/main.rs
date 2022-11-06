#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_json;

extern crate chrono;
extern crate dotenv;

pub mod email_service;
pub mod handlers;
pub mod models_bk;
mod db;
mod routes;
mod schema;
mod error;
mod models;
mod prelude;

use actix::Addr;
use actix_identity::IdentityMiddleware;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

use actix_cors::Cors;
use actix_web::{http, middleware, web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};

pub struct AppState {
    pub db: Addr<db::DbExecutor>,
}

// Tokio-based single-threaded async runtime for the Actix ecosystem.
// To achieve similar performance to multi-threaded, work-stealing runtimes, applications using actix-rt will create multiple, mostly disconnected, single-threaded runtimes.
// This approach has good performance characteristics for workloads where the majority of tasks have similar runtime expense.
// https://docs.rs/actix-rt/latest/actix_rt/index.html
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");
    // Start http server
    HttpServer::new(move || {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let allowed_origin: String =
            std::env::var("ALLOWED_ORIGIN").unwrap_or_else(|_| "*".to_string());

        // create db connection pool
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: lib_authentication::db::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let cors = Cors::default()
            .allowed_origin_fn(move |origin, _req_head| {
                origin.as_bytes().ends_with(allowed_origin.as_bytes())
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::ACCEPT, http::header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool))
            .wrap(cors)
            .wrap(IdentityMiddleware::default())
            .wrap(middleware::Logger::default())
            .wrap(lib_authentication::auth::middleware::Authentication::new(
                lib_authentication::auth::SECRET_KEY.as_bytes(),
                &routes::IGNORE_ROUTES,
            ))

            .configure(routes::config_services)
            .app_data(web::JsonConfig::default().limit(4096))
    })
    .bind(&bind_address)
    .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address))
    .run()
    .await
}
