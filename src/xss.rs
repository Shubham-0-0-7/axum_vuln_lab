use axum::{
    extract::Query,
    response::Html,
};
use std::collections::HashMap;

pub async fn xss() -> Html<&'static str> {
    Html(include_str!("../templates/xss.html"))
}

pub async fn reflect_xss(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    let input = params.get("q").map(String::as_str).unwrap_or("");
    Html(format!(
        r#"
        <html>
            <body style="font-family: sans-serif; padding: 10px; background: #fff;">
                <h3>Reflected Output:</h3>
                <div>{}</div>
            </body>
        </html>
        "#,
        input
    ))
}
