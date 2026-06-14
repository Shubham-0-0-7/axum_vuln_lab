use axum::{
    extract::Query,
    response::Html,
};
use std::collections::HashMap;
use html_escape::encode_text;

pub async fn reflect_xss_safe(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    let input = params.get("q").map(String::as_str).unwrap_or("");
    let safe_input = encode_text(input);
    Html(format!(
        r#"
        <html>
            <body style="font-family: sans-serif; padding: 10px; background: #fff;">
                <h3>Reflected Output:</h3>
                <div>{}</div>
            </body>
        </html>
        "#,
        safe_input
    ))
}
