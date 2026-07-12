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
    let email:String = conn.query_row("SELECT email FROM profiles WHERE username='alice'", 
        [], |row| row.get(0)).unwrap_or_else(
        |_| "alice@example.com".to_string());
    email
}

pub async fn update_email(headers: HeaderMap, Form(params): Form<UpdateEmailParams>)->impl IntoResponse{
    let has_cookie = headers.get(COOKIE).map(|v| v.to_str().unwrap_or("")).contains("session_user=alice").unwrap_or(false);
    if !has_cookie{
        return (StatusCode::UNAUTHORIZED, "unauthorized:no active session cookie found!".into_response());
    }

    let conn = match Connection::open("db/app.db"){
        Ok(c) => c,
        Err(err)=>return (StatusCode::INTERNAL_SERVER_ERROR, format!("database error: {}", err)).into_response(),};
    match conn.execute("UPDATE profiles SET email = ? WHERE username = 'alice'", [&params.email]) {
        Ok(_) => (StatusCode::OK, format!("Success: Email updated to {}", params.email)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to update database: {}", err)).into_response(),
    }
}

pub fn init_db() {
    let conn = Connection::open("db/app.db").expect("failed to open database");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS profiles (
            username TEXT PRIMARY KEY,
            email TEXT NOT NULL
        )",
        [],
    ).expect("failed to create profiles table");
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM profiles", [], |row| row.get(0))
        .unwrap_or(0);
    if count == 0 {
        conn.execute(
            "INSERT INTO profiles (username, email) VALUES ('alice', 'alice@example.com')",
            [],
        ).expect("failed to seed profiles table");
        println!("[+] seeded profiles table in db/app.db");
    }
}
