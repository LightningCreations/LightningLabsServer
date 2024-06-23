use diesel::{insert_into, prelude::*};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{db::establish_connection, models::Entity};
use diesel_async::RunQueryDsl;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use uuid::Uuid;

#[openapi]
#[get("/<collection>")]
pub async fn get_x(collection: &str) -> Json<Vec<Uuid>> {
    let connection = &mut establish_connection().await;

    Json({
        use crate::schema::entities::dsl::*;
        entities
            .filter(ty.eq(collection))
            .select(id)
            .load(connection)
            .await
            .expect("error loading entities")
    })
}

#[openapi]
#[get("/<collection>/<item>")]
pub async fn get_xy(collection: &str, item: Uuid) -> Json<Entity> {
    let connection = &mut establish_connection().await;

    Json({
        use crate::schema::entities::dsl::*;
        entities
            .filter(ty.eq(collection))
            .find(item)
            .select(Entity::as_select())
            .first(connection)
            .await
            .expect("error loading entity")
    })
}

#[openapi]
#[get("/<super_collection>/<parent>/<collection>")]
pub async fn get_xyz(super_collection: &str, parent: Uuid, collection: &str) -> Json<Vec<Uuid>> {
    let _ = super_collection; // :) deal with it later
    let connection = &mut establish_connection().await;

    Json({
        use crate::schema::entities::dsl::*;
        entities
            .filter(ty.eq(collection))
            .filter(parent_id.eq(parent))
            .select(id)
            .load(connection)
            .await
            .expect("error loading entities")
    })
}

#[derive(Deserialize, JsonSchema)]
pub struct ReceivedEntity {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub owner_id: Uuid,
    pub dacl_id: Uuid,
    #[serde(flatten)]
    pub payload: serde_json::Value,
}

#[derive(Serialize, JsonSchema)]
pub struct PostResponse {
    status: String,
}

#[openapi]
#[post("/<collection>", data = "<body>")]
pub async fn post_x(collection: &str, body: Json<ReceivedEntity>) -> Json<PostResponse> {
    let entity = Entity {
        id: body.id,
        parent_id: body.parent_id.unwrap(),
        owner_id: body.owner_id,
        dacl_id: body.dacl_id,
        ty: collection.into(),
        payload: body.payload.clone(),
    };

    let connection = &mut establish_connection().await;

    {
        use crate::schema::entities::dsl::*;
        insert_into(entities)
            .values(entity)
            .execute(connection)
            .await
            .expect("error inserting new data");
    }

    Json(PostResponse {
        status: "ok".into(),
    })
}

#[openapi]
#[post("/<super_collection>/<parent>/<collection>", data = "<body>")]
pub async fn post_xyz(
    super_collection: &str,
    parent: Uuid,
    collection: &str,
    body: Json<ReceivedEntity>,
) -> Json<PostResponse> {
    let _ = super_collection; // :) deal with it later
    let entity = Entity {
        id: body.id,
        parent_id: parent, // Yeet the provided parent out the window
        owner_id: body.owner_id,
        dacl_id: body.dacl_id,
        ty: collection.into(),
        payload: body.payload.clone(),
    };

    let connection = &mut establish_connection().await;

    {
        use crate::schema::entities::dsl::*;
        insert_into(entities)
            .values(entity)
            .execute(connection)
            .await
            .expect("error inserting new data");
    }

    Json(PostResponse {
        status: "ok".into(),
    })
}
