use axum::{response::Html, Router, routing::{get, put, options}};
use tokio::net::TcpListener;
use axum::http::{HeaderMap, HeaderValue};

pub async fn cors_page() -> Html<&'static str> {
    Html(include_str!("../templates/cors.html"))
}

pub async fn get_resources() -> (HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
    //new concept learnt 
    (headers, r#"{"message": "hello from the api server"}"#) //implicit return of tuple ... doesnt require semicolon or "return"
    // return (headers, r#"{"message": "hello from the api server"}"#);     
    // anf this is explicit return
}

pub async fn update_resources()-> (HeaderMap, &'static str){
    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
    (headers, r#"{"message":"resources updated successfully"}"#)
}

pub async fn handle_options() -> HeaderMap{
    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
    headers.insert("Access-Control-Allow-Methods", HeaderValue::from_static("GET, PUT, OPTIONS"));
    headers.insert("Access-Control-Allow-Headers", HeaderValue::from_static("Content-Type"));
    headers
}

pub async fn start_api_server() {
    let api_router = Router::new().route("/api/resource", 
     get(get_resources)
    .put(update_resources)
    .options(handle_options));
    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
    println!("API Server running on http://127.0.0.1:3001");
    axum::serve(listener, api_router).await.unwrap();
}