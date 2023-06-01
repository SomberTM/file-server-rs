-- Your SQL goes here
DROP TABLE files;

CREATE TABLE files (
    id uuid PRIMARY KEY,
    name VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL,
    organization_id uuid NOT NULL REFERENCES organizations (id) ON DELETE CASCADE
);