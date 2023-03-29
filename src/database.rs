pub mod api_connector{

    use reqwest;
    use actix_web::{HttpResponse, Responder};
    use serde_json::Value;
    use chrono::prelude::*;
    use crate::models::repository::*;

    pub async fn create_client_get_request(url: String) -> impl Responder {
        let client = reqwest::Client::new();
        let res = client.get(url)
            .header("Accept","application/json")
            .header("NS","base")
            .header("DB","base")
            .basic_auth("root", Some("root"))
            .send()
            .await
            .unwrap();

        match res.json().await {
            Ok(json) =>
                {
                    let body: Value = json;
                    HttpResponse::Ok().json(body)
                },
            Err(e) => HttpResponse::Ok().json(format!("error {}", e))
        }
    }

    pub async fn create_client_post_request(req: PostRepository) -> impl Responder{
        let repo = Repository {
            id: String::from(""),
            name: String::from(&req.name),
            created_at: format!("{}", Local::now()),
            license: String::from(&req.license),
            primary_language: String::from(""),
            commit_count: 0,
            forks_count: 0,
            pull_requests: 0,
            stars_count: 0,
            watchers: 0,
            languages_used: Vec::new(),
        };
        let client = reqwest::Client::new();
        let res = client.post("http://127.0.0.1:8000/key/repository")
            .header("Accept","application/json")
            .header("NS","base")
            .header("DB","base")
            .basic_auth("root", Some("root"))
            .json(&repo)
            .send()
            .await
            .expect("Error: Database not reachable.");
        let res = handle_response(res).await;
        res
    }

    pub async fn create_client_put_request(req: Repository, id: String) -> impl Responder{
        let client = reqwest::Client::new();
        let url = format!("http://127.0.0.1:8000/key/repository/{id}");
        let res = client.put(url)
            .header("Accept","application/json")
            .header("NS","base")
            .header("DB","base")
            .basic_auth("root", Some("root"))
            .json(&req)
            .send()
            .await
            .expect("Error: Database not reachable.");
        let res = handle_response(res).await;
        res
    }

    pub async fn create_client_delete_request(id: String) -> impl Responder{
        let client = reqwest::Client::new();
        let url = format!("http://127.0.0.1:8000/key/repository/{id}");
        let res = client.delete(url)
            .header("Accept","application/json")
            .header("NS","base")
            .header("DB","base")
            .basic_auth("root", Some("root"))
            .send()
            .await
            .expect("Error: Database not reachable.");
        let res = handle_response(res).await;
        res
    }

    async fn handle_response(res: reqwest::Response) -> impl Responder{
        match res.status() {
            reqwest::StatusCode::OK => {
                match res.json().await {
                    Ok(json) => {
                        let body: Value = json;
                        HttpResponse::Created().json(body)
                    },
                    Err(e) => HttpResponse::InternalServerError()
                        .json(format!("Json parsing error: {}", e)),
                }
            },
            reqwest::StatusCode::FORBIDDEN => {
                HttpResponse::Forbidden()
                    .json("You are not allowed to access this resource")
            },
            other => {
                HttpResponse::InternalServerError()
                    .json(format!("Something went wrong: {}", other))
            },
        }
    }
}

