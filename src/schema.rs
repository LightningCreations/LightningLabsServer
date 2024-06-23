// @generated automatically by Diesel CLI.

diesel::table! {
    entities (id) {
        id -> Uuid,
        parent_id -> Uuid,
        owner_id -> Uuid,
        dacl_id -> Uuid,
        ty -> Varchar,
        payload -> Jsonb,
    }
}

diesel::table! {
    logins (id) {
        id -> Uuid,
        email -> Varchar,
        hash -> Bytea,
        salt -> Bytea,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    entities,
    logins,
);
