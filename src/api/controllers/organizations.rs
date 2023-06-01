use crate::models::{File, NewFile, NewOrganization, Organization};
use crate::DbPool;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use chrono::Utc;
use diesel::prelude::*;

// TODO: ERRORS NEED TO PROPOGATE THROUGH web::block CALLS FOR PROPER HttpResponse STATUS CODE.

// These functions are scoped to /organizations

#[get("")]
pub async fn get_organizations(pool: web::Data<DbPool>) -> impl Responder {
    let get_organizations_result = web::block(move || {
        let connection = &mut pool
            .get()
            .expect("Couldn't get database connection from pool");

        use crate::schema::organizations::dsl::organizations;

        organizations
            .load::<Organization>(connection)
            .expect("Error loading organizations")
    })
    .await;

    match get_organizations_result {
        Ok(organizations) => HttpResponse::Ok().json(organizations),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{organization_id}")]
pub async fn get_organization(
    pool: web::Data<DbPool>,
    path: web::Path<(String,)>,
) -> impl Responder {
    let (organization_id,) = path.into_inner();
    let organization_uuid = uuid::Uuid::parse_str(&organization_id);

    if let Err(_) = organization_uuid {
        return HttpResponse::BadRequest().finish();
    }

    let organization_uuid = organization_uuid.unwrap();

    let get_organization_result = web::block(move || {
        let connection = &mut pool
            .get()
            .expect("Couldn't get database connection from pool");

        use crate::schema::organizations::dsl::{id, organizations};

        organizations
            .filter(id.eq(organization_uuid))
            .first::<Organization>(connection)
            .expect("Couldn't find organization") as Organization
    })
    .await;

    match get_organization_result {
        Ok(organization) => HttpResponse::Ok().json(organization),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("")]
pub async fn create_organization(
    pool: web::Data<DbPool>,
    json: web::Json<NewOrganization>,
) -> impl Responder {
    let create_organization_result = web::block(move || {
        let connection = &mut pool
            .get()
            .expect("Couldn't get database connection from pool");

        let new_organization: Organization = Organization {
            id: uuid::Uuid::new_v4(),
            name: json.into_inner().name,
            created_at: Utc::now().naive_utc(),
        };

        use crate::schema::organizations::dsl::organizations;

        diesel::insert_into(organizations)
            .values(&new_organization)
            .get_result::<Organization>(connection)
            .expect("Error creating organization")
    })
    .await;

    match create_organization_result {
        Ok(new_organization) => HttpResponse::Ok().json(new_organization),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/{organization_id}")]
pub async fn update_organization(
    pool: web::Data<DbPool>,
    json: web::Json<NewOrganization>,
    path: web::Path<(String,)>,
) -> impl Responder {
    let (organization_id,) = path.into_inner();
    let organization_uuid = uuid::Uuid::parse_str(&organization_id);

    if let Err(_) = organization_uuid {
        return HttpResponse::BadRequest().finish();
    }

    let organization_uuid = organization_uuid.unwrap();

    let new_name = json.into_inner().name;

    let update_organization_result = web::block(move || {
        let connection = &mut pool
            .get()
            .expect("Couldn't get database connection from pool");

        use crate::schema::organizations::dsl::{name, organizations};

        diesel::update(organizations.find(organization_uuid))
            .set(name.eq(new_name))
            .get_result::<Organization>(connection)
            .expect("Could not update organization")
    })
    .await;

    match update_organization_result {
        Ok(organization) => HttpResponse::Ok().json(organization),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/{organization_id}")]
pub async fn delete_organization(
    pool: web::Data<DbPool>,
    path: web::Path<(String,)>,
) -> impl Responder {
    let (organization_id,) = path.into_inner();
    let organization_uuid = uuid::Uuid::parse_str(&organization_id);

    if let Err(_) = organization_uuid {
        return HttpResponse::BadRequest().finish();
    }

    let organization_uuid = organization_uuid.unwrap();

    let delete_organization_result = web::block(move || {
        let connection = &mut pool
            .get()
            .expect("Couldn't get database connection from pool");

        use crate::schema::organizations::dsl::{id, organizations};

        diesel::delete(organizations.filter(id.eq(organization_uuid)))
            .get_result::<Organization>(connection)
            .expect("Error deleting organization")
    })
    .await;

    match delete_organization_result {
        Ok(organization) => HttpResponse::Ok().json(organization),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// These functions are scoped to /organizations/{organization_id}/files

#[get("")]
pub async fn get_organization_files(
    pool: web::Data<DbPool>,
    path: web::Path<(String,)>,
) -> impl Responder {
    let (organization_id,) = path.into_inner();
    let organization_uuid = uuid::Uuid::parse_str(&organization_id);
    drop(organization_id);

    if let Err(_) = organization_uuid {
        return HttpResponse::BadRequest().finish();
    }

    let organization_uuid = organization_uuid.unwrap();

    let get_organization_files_result = web::block(move || {
        let connection = &mut pool
            .get()
            .expect("Couldn't get database connection from pool");

        use crate::schema::files::dsl::{files, organization_id};

        files
            .filter(organization_id.eq(organization_uuid))
            .load::<File>(connection)
            .expect("Error fetching files") as Vec<File>
    })
    .await;

    match get_organization_files_result {
        Ok(files) => HttpResponse::Ok().json(files),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{file_id}")]
pub async fn get_organization_file(
    pool: web::Data<DbPool>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    dbg!(&path.into_inner());
    HttpResponse::NotImplemented().finish()
}

#[post("")]
pub async fn create_organization_file(
    pool: web::Data<DbPool>,
    path: web::Path<(String,)>,
    json: web::Json<NewFile>,
) -> impl Responder {
    let (organization_id,) = path.into_inner();
    let organization_uuid = uuid::Uuid::parse_str(&organization_id);
    drop(organization_id);

    if let Err(_) = organization_uuid {
        return HttpResponse::BadRequest().finish();
    }

    let organization_uuid = organization_uuid.unwrap();

    let create_organization_file_result = web::block(move || {
        let connection = &mut pool
            .get()
            .expect("Couldn't get database connection from pool");

        use crate::schema::files::dsl::files;

        let new_file: File = File {
            id: uuid::Uuid::new_v4(),
            name: json.into_inner().name,
            organization_id: organization_uuid,
            created_at: Utc::now().naive_utc(),
        };

        diesel::insert_into(files)
            .values(new_file)
            .get_result::<File>(connection)
            .expect("Error creating file")
    })
    .await;

    match create_organization_file_result {
        Ok(file) => HttpResponse::Ok().json(file),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/{file_id}")]
pub async fn update_organization_file(
    pool: web::Data<DbPool>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    dbg!(&path.into_inner());
    HttpResponse::NotImplemented().finish()
}

#[delete("/{file_id}")]
pub async fn delete_organization_file(
    pool: web::Data<DbPool>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    dbg!(&path.into_inner());
    HttpResponse::NotImplemented().finish()
}
