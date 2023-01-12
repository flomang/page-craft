
// use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
// use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
// use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

// mod starwars;
// mod db;

// use starwars::{QueryRoot, StarWars, StarWarsSchema};
// use std::env;

// async fn index(schema: web::Data<StarWarsSchema>, req: GraphQLRequest) -> GraphQLResponse {
//     schema.execute(req.into_inner()).await.into()
// }

// async fn index_graphiql() -> Result<HttpResponse> {
//     Ok(HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(GraphiQLSource::build().endpoint("/").finish()))
// }


// #[actix_web::main]
// pub async fn start_server() -> std::io::Result<()> {
//     //let frontend_origin = env::var("FRONTEND_ORIGIN").ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let database_pool = new_pool(database_url).expect("Failed to create pool.");
//     let database_address =
//         SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));

//     //let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");
//     let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
//         .data(StarWars::new())
//         .finish();

//     println!("GraphiQL IDE: http://localhost:8000");

//     HttpServer::new(move || {
//         App::new()
//             .app_data(Data::new(schema.clone()))
//             .service(web::resource("/").guard(guard::Post()).to(index))
//             .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
//     })
//     .bind("127.0.0.1:8000")?
//     .run()
//     .await
// }

// fn main() {
//     start_server()
// }
#![allow(unused_must_use)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod app;
mod blog;
mod db;
mod error;
mod models;
mod prelude;
mod schema;
mod starwars;
mod utils;

use std::env;

fn main() {
    dotenv::dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "pagecraft=debug,actix_web=info");
    }
    env_logger::init();
    app::start_server();
}