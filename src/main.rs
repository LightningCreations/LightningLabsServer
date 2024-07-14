use std::net::Ipv4Addr;

use dotenvy::dotenv;
use rocket::{http::Method, Config};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_okapi::{
    openapi_get_routes,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};
// use rocket::http::Method;
// use rocket_cors::{AllowedOrigins, CorsOptions};

pub mod db;
pub mod models;
pub mod routes;
pub mod schema;

async fn initialize() {
    dotenv().ok();
    // nothing yet
}

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "../api/openapi.json".to_owned(),
        ..Default::default()
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    initialize().await;

    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Get, Method::Post, Method::Patch]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .allow_credentials(true);

    let _rocket = rocket::build()
        .configure(Config {
            address: Ipv4Addr::new(0, 0, 0, 0).into(),
            ..Config::default()
        })
        .mount(
            "/api",
            openapi_get_routes![
                routes::get_x,
                routes::get_xy,
                routes::get_xyz,
                routes::post_x,
                routes::post_xyz,
            ],
        )
        .mount("/swagger-ui/", make_swagger_ui(&get_docs()))
        .attach(cors.to_cors().unwrap())
        .launch()
        .await?;

    Ok(())
}
