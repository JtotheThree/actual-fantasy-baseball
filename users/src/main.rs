#[macro_use]
extern crate log;

mod auth;
mod config;
mod graphql;
mod models;

use crate::config::CONFIG;
use crate::graphql::{index, Mutation, Query};
use crate::models::User;

use actix_web::{middleware, web, App, HttpServer};
use async_graphql::{
    extensions::{apollo_persisted_queries::ApolloPersistedQueries, ApolloTracing, Logger, apollo_persisted_queries::LruCacheStorage},
    EmptySubscription, Schema,
};
use wither::mongodb::{Client, Database};
use wither::Model;


// TODO: Make all auth look ups skip mongo and only use redis
// TODO: Put all auth checks into an is_verified common library
// TODO: Ensure session look ups are async???


pub type UsersSchema = Schema<Query, Mutation, EmptySubscription>;


async fn init_db() -> Database {
    let db = Client::with_uri_str(&CONFIG.database.url)
        .await
        .expect("Cannot connect to the db")
        .database(&CONFIG.database.name);

    info!("Mongo database initialized");

    User::sync(&db)
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

fn init_graphql(db: &Database, redis: &redis::Client) -> UsersSchema {
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