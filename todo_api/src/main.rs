use std::process::exit;
use clap::Parser;
use cli::Args;
use db::{cors::Cors, Database};
use logger::Logger;
use rocket::launch;
use rocket_okapi::{openapi_get_routes, swagger_ui::{make_swagger_ui, SwaggerUIConfig}};
use routes::tasks::*;

pub mod cli;
pub mod db;
pub mod error;
pub mod logger;
pub mod models;
pub mod routes;

#[launch]
async fn rocket() -> _ {
    Logger::init();

    let args = Args::parse();

    let db = Database::new(args.addr, &args.username, &args.passwd).await
        .unwrap_or_else(|e| {
            log::error!("Failed to connect to the database: {e}");
            exit(1);
        });  

    rocket::build()
        .mount(
            "/api",
            openapi_get_routes![
                tasks,
                tasks_create,
                tasks_delete,  
                tasks_update,         
            ],
        )
        .mount(
            "/swagger",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../api/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .manage(db)
        .attach(Cors)
}