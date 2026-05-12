use axum::{
    extract::{Form, Query},
    response::Html,
};

use rusqlite::Connection;
use serde::Deserialize;
use std::collections::HashMap;

pub async fn index() -> Html<&'static str> {
    Html(include_str!("../templates/index.html"))
}

pub async fn admin() -> Html<&'static str> {
    Html(include_str!("../templates/admin.html"))
}

#[derive(Deserialize)]
pub struct Input {
    pub username: String,
    pub password: String,
}

pub async fn login_safe(Form(input): Form<Input>) -> String {
    let conn = Connection::open("db/app.db").unwrap();

    let mut stmt = conn.prepare("SELECT * FROM users WHERE username = ? AND password = ?").unwrap();
    let mut rows = stmt
        .query([&input.username, &input.password])
        .unwrap();

    if rows.next().unwrap().is_some() {
        return "[+] Secure Login Successful!".to_string();
    }
    "[-] Login Failed".to_string()
}

pub async fn search_safe(
    Query(params): Query<HashMap<String, String>>,
) -> String {
    let conn = Connection::open("db/app.db").unwrap();

    let query = params
        .get("q")
        .map(String::as_str)
        .unwrap_or("");

    let mut stmt = conn.prepare("SELECT name, description FROM products WHERE name = ?").unwrap();
    let rows = stmt.query_map([query], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
        ))
    });
    
    match rows{
        Ok(products) => {
            let mut output = String::new();
            for product in products {
                let (name, description) = product.unwrap();
                output.push_str(&format!(
                    "Product: {}\nDescription: {}\n\n",
                    name,
                    description
                ));
            }
            if output.is_empty() {
                output.push_str("No products found");
            }
            output
        }
        Err(err) => format!("SQL Error: {}", err),
    }
}