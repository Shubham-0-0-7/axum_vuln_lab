use axum::{
    extract::Form, 
    http::{HeaderMap, HeaderValue, StatusCode,
    header::{COOKIE, SET_COOKIE}},
    response::{Html, IntoResponse},
};

// https://stackoverflow.com/questions/2581488/understanding-csrf
// you read this once bro i mean ... what an explanation!!

use rusqlite::Connection;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateEmailParams{
    pub email:String,
}

pub async fn csrf_page(headers: HeaderMap)->impl IntoResponse{
    let mut response_headers = HeaderMap::new();
    let has_cookie = headers.get(COOKIE).map(|v| v.to_str().unwrap_or("")).contains("session_user=alice").unwrap_or(false);
    if !has_cookie{
        response_headers.insert(SET_COOKIE, HeaderValue::from_static("session_user=alice;Path=/"));

        return (response_headers, Html(include_str!("../templates/csrf.html")))
    }
}

pub async fn get_profile_email() -> String{
    let conn = Connection::open("db/app.db").expect("failed to open database");
    let email:String = conn.query_row("SELECT email FROM profiles WHERE username='alice'", [], |row| row.get(0)),unwrap_or_else(
        |_| "alice@example.com".to_string());
    email
}