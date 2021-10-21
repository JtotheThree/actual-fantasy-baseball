#[macro_use]
extern crate log;

mod config;
mod graphql;
mod models;
mod routes;

use crate::config::CONFIG;
use crate::graphql::{AppSchema, Mutation, Query};
use crate::models::Team;
use crate::routes::index;

use actix_web::{middleware, web, App, HttpServer};
use async_graphql::{
    extensions::{apollo_persisted_queries::ApolloPersistedQueries, ApolloTracing, Logger, apollo_persisted_queries::LruCacheStorage},
    EmptySubscription, Schema,
};
use wither::mongodb::{Client, Database};
use wither::Model;


async fn init_db() -> Database {
    let db = Client::with_uri_str(&CONFIG.database.url)
        .await
        .expect("Cannot connect to the db")
        .database(&CONFIG.database.name);

    info!("Mongo database initialized");

    Team::sync(&db)
        .await
        .expect("Failed syncing indexes");

    db
}

async fn init_redis() -> redis::Client {
    let addr = format!("redis://{}", CONFIG.redis.url);

    let client: redis::Client = redis::Client::open(addr).unwrap();

    info!("Redis client initialised");

    client
}

fn init_graphql(db: &Database, redis: &redis::Client) -> AppSchema {
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(db.clone())
        .data(redis.clone())
        .extension(ApolloTracing)
        .extension(ApolloPersistedQueries::new(LruCacheStorage::new(256)))
        .extension(Logger)
        .finish();

    info!("Initialised graphql");

    schema
}

fn init_logger() {
    if CONFIG.debug {
        std::env::set_var("RUST_BACKTRACE", "1");
        std::env::set_var("RUST_LOG", "info,actix_web=info,actix_redis=info");
    }

    pretty_env_logger::init();
    info!("Logger initialised");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("called main()");
    init_logger();

    let db = init_db().await;
    let redis = init_redis().await;
    let schema = init_graphql(&db, &redis);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(db.clone())
            .data(schema.clone())
            .service(
                web::resource("/").route(web::post().to(index))
            )
    })
    .bind(format!("0.0.0.0:{:?}", CONFIG.server.port))?
    .run()
    .await
}
