pub mod api;
pub mod file_manager;
pub mod models;
pub mod schema;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use diesel::{r2d2, PgConnection};
use dotenvy::dotenv;
use std::{env, io};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let filestore_dir = env::var("LOCAL_FILESTORE_DIR").expect("LOCAL_FILESTORE_DIR must be set");

    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("DATABASE_URL should be a valid PostgreSQL connection string");

    use api::routes;

    // Not sure how much I like the nested scopes like this.
    // It works for now but there is probably a simpler/more
    // understandable solution.

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin_fn(|_, _| true)
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]),
            )
            .service(
                web::scope("/organizations")
                    .service(routes::organizations::get_organization)
                    .service(routes::organizations::get_organizations)
                    .service(routes::organizations::create_organization)
                    .service(routes::organizations::update_organization)
                    .service(routes::organizations::delete_organization)
                    .service(
                        web::scope("/{organization_id}/files")
                            .service(routes::organizations::get_organization_files)
                            .service(routes::organizations::upload_organization_files),
                    ),
            )
            .service(
                Files::new("/fileserver", format!("{}/", filestore_dir.clone())).prefer_utf8(true),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
