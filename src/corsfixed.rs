use axum::{
    response::{Html, IntoResponse},
    http::{HeaderMap, HeaderValue, header::ORIGIN},
};

const ALLOWED_ORIGINS: &[&str] = &["http://127.0.0.1:3000","http://localhost:3000"];

pub async fn corsfixed_page() -> Html<&'static str>{
    Html(include_str!("../templates/corsfixed.html"))
}

pub async fn get_resources_safe(headers:HeaderMap) -> impl IntoResponse{
    let mut response_headers = HeaderMap::new();

    if let Some(origin) = headers.get(ORIGIN){
        if let Ok(origin_str) = origin.to_str(){
            if ALLOWED_ORIGINS.contains(&origin_str){
                if let Ok(val) = HeaderValue::from_str(origin_str){
                    response_headers.insert("Access-Control-Allow-Origin", val);
                }
            }
        }
    }
    (response_headers, r#"{"hello from the secure api server"}"#)
}

pub async fn update_resources_safe(headers: HeaderMap) -> impl IntoResponse {
    let mut response_headers = HeaderMap::new();
    if let Some(origin) = headers.get(ORIGIN){
        if let Ok(origin_str) = origin.to_str(){
            if ALLOWED_ORIGINS.contains(&origin_str){
                if let Ok(val) = HeaderValue::from_str(origin_str){
                    response_headers.insert("Access-Control-Allow-Origin",val);
                }
            }
        }
    }
    (response_headers, r#"{"message": "resources updated successfully"}"#)
}

pub async fn handle_options_safe(headers: HeaderMap) -> impl IntoResponse{
    let mut response_headers = HeaderMap::new();

    if let Some(origin) = headers.get(ORIGIN){
        if let Ok(origin_str) = origin.to_str(){
            if ALLOWED_ORIGINS.contains(&origin_str){
                if let Ok(val) = HeaderValue::from_str(origin_str){
                    response_headers.insert("Access-Control-Allow-Origin", val);
                    response_headers.insert("Access-Control-Allow-Methods", HeaderValue::from_static("GET, PUT, OPTIONS"));
                    response_headers.insert("Access-Control-Allow-Headers", HeaderValue::from_static("Content-Type"));
                }
            }
        }
    }
    response_headers
}
