use axum::{
    extract::Query,
    response::Html,
};
use std::collections::HashMap;
use html_escape::encode_text;

pub async fn xss_safe(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    let input = params.get("q").map(String::as_str).unwrap_or("");
    let safe_input = encode_text(input);
    Html(format!(
        r#"
        <html>
            <body>
                <h1>no reflected xss</h1>
                <p>you searched for: {}</p>

                <form action="/xss-safe">
                    <input type="text" name="q">
                    <button type="submit">search</button>
                </form>
            </body>
        </html>
        "#,
        safe_input
    ))
}
