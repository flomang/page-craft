use crate::db::{new_pool, DbExecutor};
use actix::prelude::{Addr, SyncArbiter};
use actix_cors::Cors;
use actix_http::header::ORIGIN;
use actix_web::{
    guard,
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    middleware::Logger,
    web,
    web::Data,
    App, HttpResponse, HttpServer, Result,
};
use std::env;

pub mod articles;
pub mod profiles;
pub mod tags;
pub mod users;

//use crate::starwars::{QueryRoot, StarWars, StarWarsSchema};
use crate::blog::{QueryRoot, MutationRoot, BlogSchema};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

//async fn index(_state: Data<AppState>, _req: HttpRequest) -> &'static str {
//    "Hello world!"
//}

async fn index(schema: web::Data<BlogSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish()))
}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    let frontend_origin = env::var("FRONTEND_ORIGIN").ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_pool = new_pool(database_url).expect("Failed to create pool.");
    let database_address =
        SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));

    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");

    HttpServer::new(move || {
        let state = AppState {
            db: database_address.clone(),
        };
        let _cors = match frontend_origin {
            Some(ref origin) if origin != "*" => Cors::default()
                .allowed_origin(origin)
                .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
                .max_age(3600),
            _ => Cors::default()
                .send_wildcard()
                .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE, ORIGIN])
                .max_age(3600),
        };

        let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(state)
        //.data(StarWars::new())
        .finish();

        App::new()
            //.app_data(Data::new(state))
            .app_data(Data::new(schema.clone()))
            .wrap(Logger::default())
            //.wrap(cors)
            .configure(routes)
    })
    .bind(&bind_address)?
    .run()
    .await
}

fn routes(app: &mut web::ServiceConfig) {
    log::info!("GraphiQL IDE: http://localhost:8000");

    app.service(web::resource("/").guard(guard::Post()).to(index))
        .service(web::resource("/").guard(guard::Get()).to(index_graphiql));

    // app.service(web::resource("/").to(index)).service(
    //     web::scope("/api")
    //         // User routes ↓
    //         .service(web::resource("users").route(web::post().to(users::register)))
    //         .service(web::resource("users/login").route(web::post().to(users::login)))
    //         .service(
    //             web::resource("user")
    //                 .route(web::get().to(users::get_current))
    //                 .route(web::put().to(users::update)),
    //         )
    //         // Profile routes ↓
    //         .service(web::resource("profiles/{username}").route(web::get().to(profiles::get)))
    //         .service(
    //             web::resource("profiles/{username}/follow")
    //                 .route(web::post().to(profiles::follow))
    //                 .route(web::delete().to(profiles::unfollow)),
    //         )
    //         // Article routes ↓
    //         .service(
    //             web::resource("articles")
    //                 .route(web::get().to(articles::list))
    //                 .route(web::post().to(articles::create)),
    //         )
    //         .service(web::resource("articles/feed").route(web::get().to(articles::feed)))
    //         .service(
    //             web::resource("articles/{slug}")
    //                 .route(web::get().to(articles::get))
    //                 .route(web::put().to(articles::update))
    //                 .route(web::delete().to(articles::delete)),
    //         )
    //         .service(
    //             web::resource("articles/{slug}/favorite")
    //                 .route(web::post().to(articles::favorite))
    //                 .route(web::delete().to(articles::unfavorite)),
    //         )
    //         .service(
    //             web::resource("articles/{slug}/comments")
    //                 .route(web::get().to(articles::comments::list))
    //                 .route(web::post().to(articles::comments::add)),
    //         )
    //         .service(
    //             web::resource("articles/{slug}/comments/{comment_id}")
    //                 .route(web::delete().to(articles::comments::delete)),
    //         )
    //         // Tags routes ↓
    //         .service(web::resource("tags").route(web::get().to(tags::get))),
    // );
}