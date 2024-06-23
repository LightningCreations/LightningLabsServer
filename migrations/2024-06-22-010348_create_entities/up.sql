-- Your SQL goes here
CREATE TABLE entities (
    id UUID PRIMARY KEY,
    parent_id UUID NOT NULL,
    owner_id UUID NOT NULL,
    dacl_id UUID NOT NULL,
    ty VARCHAR NOT NULL,
    payload JSONB NOT NULL
)
