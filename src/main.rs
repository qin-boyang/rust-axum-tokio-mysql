use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::{get, post};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let route = Router::new()
        .route("/", get(root))
        .route("/user/{id}", get(fetch_user_by_id))
        .route("/user/details", post(fetch_user_by_id_post_method));
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, route).await.unwrap();
}

#[derive(serde::Serialize)]
struct User<'a> {
    id: u32,
    name: &'a str
}
#[derive(serde::Deserialize)]
struct UserId {
    id: u32
}
async fn fetch_user_by_id(Path(user_id): Path<u32>) -> impl IntoResponse {
    Json(User { id: user_id, name: "Bob" })
}

async fn fetch_user_by_id_post_method(Json(user): Json<UserId>)-> impl IntoResponse {
    Json(User { id: user.id, name: "Bob" })
}
async fn root() -> impl IntoResponse {
    "Hello, World!"
}