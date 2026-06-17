mod sqli;
mod sqlifixed;
mod xss;
mod xssfixed;
mod cors;
mod corsfixed;
mod cmdi;
mod cmdifixed;
mod lfi;
mod lfifixed;
mod ssrf;
mod ssrffixed;

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
        .route("/xss/reflect", get(xss::reflect_xss))
        .route("/xss-safe/reflect", get(xssfixed::reflect_xss_safe))
        .route("/cors", get(cors::cors_page))
        .route("/corsfixed", get(corsfixed::corsfixed_page))
        .route("/cmdi", get(cmdi::cmdi_page))
        .route("/cmdi/ping", post(cmdi::ping))
        .route("/cmdifixed", get(cmdifixed::cmdifixed_page))
        .route("/cmdifixed/ping", post(cmdifixed::ping_safe))
        .route("/lfi", get(lfi::lfi_page))
        .route("/lfi/read", get(lfi::read_file))
        .route("/lfi-safe/read", get(lfifixed::read_file_safe))
        .route("/ssrf", get(ssrf::ssrf_page))
        .route("/ssrf/fetch", get(ssrf::fetch_url))
        .route("/ssrf-safe/fetch", get(ssrffixed::fetch_url_safe));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");
    tokio::spawn(cors::start_api_server());
    axum::serve(listener, app).await.unwrap();
}