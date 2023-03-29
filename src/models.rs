pub mod repository {

    use serde::{Serialize, Deserialize};
    use utoipa::ToSchema;

    #[derive(Serialize, Deserialize, ToSchema)]
    pub struct Repository {
        pub id: String,
        pub commit_count: i32,
        pub created_at: String,
        pub forks_count: i32,
        pub languages_used: Vec<String>,
        pub license: String,
        pub name: String,
        pub primary_language: String,
        pub pull_requests: i32,
        pub stars_count: i32,
        pub watchers: i32,
    }

    #[derive(Serialize, Deserialize, ToSchema)]
    pub struct PostRepository {
        pub name: String,
        pub license: String,
    }

    #[derive(Serialize, Deserialize, ToSchema)]
    pub struct ResponseInfo {
        pub time: String,
        pub status: String,
        pub result: Vec<Repository>,
    }
}
