use axum::extract::Form;
use std::process::Command;
use serde::Deserialize;
use axum::response::Html;

#[derive(Deserialize)]
pub struct PingParameters{
    pub host: String,
}

pub async fn cmdifixed_page() -> Html<&'static str> {
    Html(include_str!("../templates/cmdi.html"))
}

pub async fn ping_safe(Form(params): Form<PingParameters>) -> String {
    let output = Command::new("ping")
        .args(["-c", "3", &params.host])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    format!("STDOUT:\n{}\n\nSTDERR:\n{}", stdout, stderr)
}
