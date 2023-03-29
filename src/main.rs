mod database;
mod models;
mod routes;
use actix_web::{ App, HttpServer };
use std::error::Error;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::routes::repository::*;
use crate::models::repository::*;


#[tokio::main]
async fn main() -> Result<(), impl Error> {
    env_logger::init();
    #[derive(OpenApi)]
    #[openapi(
        paths(get_repositories, get_repository, add_repository, change_repository, delete_repository),
        components(schemas(ResponseInfo, PostRepository, Repository)
        )
    )]
    struct ApiDoc;

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_repository)
            .service(get_repository)
            .service(add_repository)
            .service(change_repository)
            .service(delete_repository)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}