// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Timestamp,
        organization_id -> Uuid,
    }
}

diesel::table! {
    organizations (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(files -> organizations (organization_id));

diesel::allow_tables_to_appear_in_same_query!(
    files,
    organizations,
);
