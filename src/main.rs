use rocket::response::status::{self, Accepted};
use rocket::serde::json::Json;
use rocket::{get, launch, post, routes};

use tracing_log::LogTracer;

use crate::controllers::database::handle_create_database;
use crate::structs::database_provider::DatabaseProvider;
use crate::utils::log::{
    LogLevel, LogType, default_logging_layer, filter_layer, json_logging_layer,
};

mod controllers;
mod structs;
mod utils;

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[post("/", format = "json", data = "<database>")]
async fn create_database<'a>(database: Json<DatabaseProvider<'a>>) -> Accepted<String> {
    let response = handle_create_database(database.into_inner()).await.unwrap();
    status::Accepted(response)
}

#[launch]
fn rocket() -> _ {
    // tracing_subscriber::fmt::init();
    use tracing_subscriber::prelude::*;

    LogTracer::init().expect("Unable to setup log tracer!");

    let log_type =
        LogType::from(std::env::var("LOG_TYPE").unwrap_or_else(|_| "formatted".to_string()));
    let log_level =
        LogLevel::from(std::env::var("LOG_LEVEL").unwrap_or_else(|_| "normal".to_string()));

    match log_type {
        LogType::Formatted => {
            tracing::subscriber::set_global_default(
                tracing_subscriber::registry()
                    .with(default_logging_layer())
                    .with(filter_layer(log_level)),
            )
            .unwrap();
        }
        LogType::Json => {
            tracing::subscriber::set_global_default(
                tracing_subscriber::registry()
                    .with(json_logging_layer())
                    .with(filter_layer(log_level)),
            )
            .unwrap();
        }
    };
    rocket::build().mount("/", routes![index, create_database])
}
