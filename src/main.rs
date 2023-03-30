mod database;
mod models;
mod routes;
use actix_web::{ web, App, HttpServer };
use std::error::Error;
use std::env;
use std::env::VarError;
use actix_web::middleware::Logger;
use log::{debug, error, warn, log_enabled, info, Level};
use env_logger::Env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::routes::repository::*;
use crate::models::repository::*;


#[tokio::main]
async fn main() -> Result<(), impl Error> {
    env::set_var("LOG_LEVEL", "info");
    let env = Env::default().filter_or("LOG_LEVEL", "info");
    env_logger::init_from_env(env);
    let url_env_var = "BASE_URL";
    match env::var(url_env_var) {
        Ok(v) => println!("BASE_URL environment variable found: {}: {}", url_env_var, v),
        Err(e) => env::set_var("BASE_URL", "http://localhost:8000")
    }

    #[derive(OpenApi)]
    #[openapi(
        paths(get_repositories, get_repository, add_repository, change_repository, delete_repository),
        components(schemas(ResponseInfo, PostRepository, Repository)))
    ]

    struct ApiDoc;

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(get_repositories)
            .service(get_repository)
            .service(add_repository)
            .service(change_repository)
            .service(delete_repository)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()))
            .wrap(Logger::default())
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}