mod vulnerable;
mod fixed;

use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(vulnerable::index))
        .route("/search", get(vulnerable::search))
        .route("/search-safe", get(fixed::search_safe))

        .route("/login", post(vulnerable::login))
        .route("/login-safe", post(fixed::login_safe));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}