use diesel::prelude::*;
use schemars::JsonSchema;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Queryable, Insertable, Selectable, JsonSchema)]
#[diesel(table_name = crate::schema::entities)]
pub struct Entity {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub owner_id: Uuid,
    pub dacl_id: Uuid,
    pub ty: String,
    #[serde(flatten)]
    pub payload: serde_json::Value,
}
