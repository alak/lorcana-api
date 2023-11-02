#[macro_use]
extern crate actix_web;
extern crate diesel;

use dotenvy::dotenv;
use std::{env, io};

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};

use actix_web::{middleware, App, HttpServer};

mod bearer_auth;
mod card;
mod constants;
mod model;
mod price;
mod request_insert_model;
mod response;
mod response_model;
mod schema;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    _ = dotenv();

    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let database_host = env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");
    let database_port = env::var("DATABASE_PORT").expect("DATABASE_PORT must be set");
    let database_user = env::var("DATABASE_USER").expect("DATABASE_USER must be set");
    let database_password = env::var("DATABASE_PWD").expect("DATABASE_PWD must be set");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let database_ssl_mode = env::var("DATABASE_SSL").expect("DATABASE_SSL must be set");
    let database_url = format!(
        "postgresql://{}:{}@{}:{}/{}?sslmode={}",
        database_user,
        database_password,
        database_host,
        database_port,
        database_name,
        database_ssl_mode
    );

    println!("url: {}", database_url);
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            // Set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(card::create)
            // .service(card::list)
            // .service(card::get)
            .service(price::get)
            .service(price::create)
            .service(price::get_all_prices)
            .service(price::all_grouped_prices)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
