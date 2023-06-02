use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::organizations;

#[derive(Queryable, Insertable, Serialize)]
#[diesel(table_name = organizations)]
pub struct Organization {
    pub id: uuid::Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct NewOrganization {
    pub name: String,
}

use crate::schema::files;

#[derive(Queryable, Insertable, Serialize)]
#[diesel(table_name = files)]
pub struct File {
    pub id: uuid::Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub url: String,
    pub organization_id: uuid::Uuid,
}

#[derive(Deserialize, Debug)]
pub struct NewFile {
    pub name: String,
}
