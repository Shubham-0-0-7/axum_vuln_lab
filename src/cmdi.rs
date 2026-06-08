use serde::Deserialize;
use axum::response::Html;
use axum::extract::Form;
use std::process::Command;

#[derive(Deserialize)]
pub struct PingParameters{
    pub host: String,
}

pub async fn ping(Form(params): Form<PingParameters>)-> String{
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("ping -c 3 {}", params.host))
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    format!("STDOUT:\n{}\n\nSTDERR:\n{}", stdout, stderr)
}
    


pub async fn cmdi_page() -> Html<&'static str>{
    Html(include_str!("../templates/cmdi.html"))

}

