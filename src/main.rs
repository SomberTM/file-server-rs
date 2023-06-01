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
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("DATABASE_URL should be a valid PostgreSQL connection string");

    use api::controllers;

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::scope("/organizations")
                .service(controllers::organizations::get_organization)
                .service(controllers::organizations::get_organizations)
                .service(controllers::organizations::create_organization)
                .service(controllers::organizations::update_organization)
                .service(controllers::organizations::delete_organization),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
