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

pub async fn login(Form(input): Form<Input>) -> String {
    let conn = Connection::open("db/app.db").unwrap();

    let vulnerable_query = format!(
        "SELECT * FROM users WHERE username='{}' AND password='{}'",
        input.username,
        input.password,
    );

    println!("{}", vulnerable_query);

    let mut stmt = match conn.prepare(&vulnerable_query) {
        Ok(stmt) => stmt,
        Err(err) => {
            return format!("SQL Error: {}", err);
        }
    };

    let mut rows = stmt.query([]).unwrap();

    if rows.next().unwrap().is_some() {
        return format!(
            "[+] Login Successful!\n\nExecuted Query:\n{}",
            vulnerable_query
        );
    }

    format!(
        "[-] Login Failed\n\nExecuted Query:\n{}",
        vulnerable_query
    )
}

pub async fn search(
    Query(params): Query<HashMap<String, String>>,
) -> String {
    let conn = Connection::open("db/app.db").unwrap();

    let query = params
        .get("q")
        .map(String::as_str)
        .unwrap_or("");

    let vulnerable_query = format!(
        "SELECT name, description FROM products WHERE name = '{}'",
        query
    );

    println!("{}", vulnerable_query);

    let mut stmt = match conn.prepare(&vulnerable_query) {
        Ok(stmt) => stmt,
        Err(err) => {
            return format!("SQL Error: {}", err);
        }
    };

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
        ))
    });

    match rows {
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