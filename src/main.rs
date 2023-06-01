pub mod api;
pub mod models;
pub mod schema;

use actix_web::{web, App, HttpServer};
use diesel::{r2d2, PgConnection};
use dotenvy::dotenv;
use std::{env, io};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dbg!("Starting main");

    dotenv().ok();
    dbg!("Loaded .env variables");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    dbg!("Loaded DATABASE_URL");

    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    dbg!("Setup connection manager");

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("DATABASE_URL should be a valid PostgreSQL connection string");
    dbg!("Built connnection pool {:?}", &pool);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(api::controllers::organizations::get_organization)
            .service(api::controllers::organizations::get_organizations)
            .service(api::controllers::organizations::create_organization)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
