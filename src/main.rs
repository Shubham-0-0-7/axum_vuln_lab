mod sqli;
mod sqlifixed;
mod xss;
mod xssfixed;
mod cors;

use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(sqli::index))
        .route("/search", get(sqli::search))
        .route("/search-safe", get(sqlifixed::search_safe))

        .route("/login", post(sqli::login))
        .route("/login-safe", post(sqlifixed::login_safe))
        .route("/xss", get(xss::xss))
        .route("/xss-safe", get(xssfixed::xss_safe))
        .route("/cors", get(cors::cors_page));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");
    tokio::spawn(cors::start_api_server());
    axum::serve(listener, app).await.unwrap();
}