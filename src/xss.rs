use axum::{
    extract::Query,
    response::Html,
};
use std::collections::HashMap;

pub async fn xss(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    let input = params.get("q").map(String::as_str).unwrap_or("");
    Html(format!(
        r#"
        <html>
            <body>
                <h1>reflected xss demo</h1>
                <p>you searched for: {}</p>

                <form action="/xss">
                    <input type="text" name="q">
                    <button type="submit">search</button>
                </form>
            </body>
        </html>
        "#,
        input
    ))
}
