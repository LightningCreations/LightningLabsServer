#[macro_use]
extern crate rocket;

use rocket_okapi::{openapi_get_routes, swagger_ui::{make_swagger_ui, SwaggerUIConfig}};

pub mod db;
pub mod models;
pub mod routes;
pub mod schema;

async fn initialize() {
    // nothing yet
}

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "../openapi.json".to_owned(),
        ..Default::default()
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    initialize().await;

    let _rocket = rocket::build().mount("/", openapi_get_routes![
        routes::get_x,
        routes::get_xy,
        routes::get_xyz,
        routes::post_x,
        routes::post_xyz,
    ]).mount("/swagger-ui/", make_swagger_ui(&get_docs())).launch().await?;

    Ok(())
}
