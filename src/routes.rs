pub mod repository {
    use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
    use crate::models::repository::*;
    use crate::database::api_connector::*;


    #[get("/")]
    pub async fn hello() -> impl Responder {
        HttpResponse::Ok().body("Git Repos REST API v1")
    }

    #[utoipa::path(responses(
    (status = 200, description = "OK", body = Vec<ResponseInfo>),
    (status = 404, description = "Files not found", body = String)))]
    #[get("/repositories")]
    pub async fn get_repositories() -> impl Responder {
        let url = String::from("http://127.0.0.1:8000/key/repository");
        let response = create_client_get_request(url).await;
        response
    }

    #[utoipa::path(responses(
    (status = 200, description = "OK", body = ResponseInfo),
    (status = 500, description = "Internal Server Error", body = String),
    (status = 404, description = "Files not found", body = String)))]
    #[get("/repositories/{id}")]
    pub async fn get_repository(path: web::Path<String>) -> impl Responder {
        let id = path.into_inner();
        let url = format!("http://127.0.0.1:8000/key/repository/{id}");
        let response = create_client_get_request(url).await;
        response
    }

    #[utoipa::path(
        request_body = PostRepository,
        responses(
            (status = 201, description = "File created successfully.", body = Vec<ResponseInfo>),
            (status = 500, description = "Internal Server Error", body = String),
            (status = 404, description = "Files not found", body = String)
            )
        )
    ]
    #[post("/repositories")]
    pub async fn add_repository(req: web::Json<PostRepository>) -> impl Responder {
        let new_repo = PostRepository {
            name: String::from(&req.name),
            license: String::from(&req.license),
        };

        let response = create_client_post_request(new_repo).await;
        response
    }

    #[utoipa::path(
        request_body = Repository,
        responses(
            (status = 204, description = "File changed successfully.", body = ResponseInfo),
            (status = 404, description = "File with specified id not found", body = String),
            (status = 409, description = "File with provided id could not be changed", body = String),
            (status = 500, description = "Internal Server Error", body = String),
            )
        )
    ]
    #[put("/repositories/{id}")]
    pub async fn change_repository(path: web::Path<String>, req: web::Json<Repository>) -> impl Responder {
        let id = path.into_inner();
        let repo = Repository {
            id: String::from(""),
            name: String::from(&req.name),
            created_at: String::from(&req.created_at),
            license: String::from(&req.license),
            primary_language: String::from(&req.primary_language),
            commit_count: req.commit_count,
            forks_count: req.forks_count,
            pull_requests: req.pull_requests,
            stars_count: req.stars_count,
            watchers: req.watchers,
            languages_used: Vec::new(),
        };

        let response = create_client_put_request(repo, id).await;
        response
    }

    #[utoipa::path(
    responses(
        (status = 201, description = "File deleted successfully.", body = ResponseInfo),
        (status = 404, description = "File with specified id not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)))
    ]
    #[delete("/repositories/{id}")]
    pub async fn delete_repository(path: web::Path<String>) -> impl Responder {
        let id = path.into_inner();
        let response = create_client_delete_request(id).await;
        response
    }
}