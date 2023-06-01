use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::ser::SerializeStruct;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::organizations;

#[derive(Queryable, Insertable)]
#[diesel(table_name = organizations)]
pub struct Organization {
    pub id: uuid::Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
}

impl Serialize for Organization {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut org = serializer.serialize_struct("Organization", 3)?;
        org.serialize_field("id", &self.id.to_string())?;
        org.serialize_field("name", &self.name)?;
        org.serialize_field("created_at", &self.created_at.to_string())?;
        org.end()
    }
}

#[derive(Queryable)]
pub struct File {
    pub id: uuid::Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub organization_id: uuid::Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct NewOrganization {
    pub name: String,
}

use crate::schema::files;

#[derive(Insertable)]
#[diesel(table_name = files)]
pub struct NewFile {
    pub name: String,
    pub created_at: NaiveDateTime,
    pub organization_id: uuid::Uuid,
}
