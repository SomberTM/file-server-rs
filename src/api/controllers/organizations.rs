use crate::models::{NewOrganization, Organization};
use crate::DbPool;

use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::Utc;
use diesel::prelude::*;

// TODO: ERRORS NEED TO PROPOGATE THROUGH web::block CALLS FOR PROPER HttpResponse STATUS CODE.

#[get("/organizations")]
pub async fn get_organizations(pool: web::Data<DbPool>) -> impl Responder {
    let get_organizations_result = web::block(move || {
        let mut connection = pool
            .get()
            .expect("Couldn't get database connection from pool");

        use crate::schema::organizations::dsl::*;

        organizations
            .load::<Organization>(&mut connection)
            .expect("Error loading organizations")
    })
    .await;

    if let Ok(orgs) = get_organizations_result {
        HttpResponse::Ok().json(orgs)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[get("/organizations/{organization_id}")]
pub async fn get_organization(
    pool: web::Data<DbPool>,
    path: web::Path<(String,)>,
) -> impl Responder {
    let (organization_id,): (String,) = path.into_inner();
    let get_organization_result = web::block(move || {
        let mut connection = pool
            .get()
            .expect("Couldn't get database connection from pool");

        use crate::schema::organizations::dsl::*;

        let organization_uuid =
            uuid::Uuid::parse_str(&organization_id).expect("Invalid UUID provided");
        organizations
            .filter(id.eq(organization_uuid))
            .first::<Organization>(&mut connection)
            .expect("Couldn't find organization") as Organization
    })
    .await;

    if let Ok(organization) = get_organization_result {
        HttpResponse::Ok().json(organization)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[post("/organizations")]
pub async fn create_organization(
    pool: web::Data<DbPool>,
    json: web::Json<NewOrganization>,
) -> impl Responder {
    let create_organization_result = web::block(move || {
        let mut connection = pool
            .get()
            .expect("Couldn't get database connection from pool");

        let new_organization: Organization = Organization {
            id: uuid::Uuid::new_v4(),
            name: json.into_inner().name,
            created_at: Utc::now().naive_utc(),
        };

        diesel::insert_into(crate::schema::organizations::table)
            .values(&new_organization)
            .get_result::<Organization>(&mut connection)
            .expect("Error creating organization")
    })
    .await;

    if let Ok(new_organization) = create_organization_result {
        HttpResponse::Ok().json(new_organization)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
