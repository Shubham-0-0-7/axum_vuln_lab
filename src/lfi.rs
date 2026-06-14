use axum::{
    extract::Query, 
    response::Html
};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct FileParams{
    pub file: String,
}

pub async fn lfi_page() -> Html<&'static str>{
    Html(include_str!("../templates/lfi.html"))
}

pub async fn read_file(Query(params): Query<FileParams>) -> String {
    let file_path = format!("templates/{}", params.file);
    match fs::read_to_string(&file_path){
        Ok(content)=>content,
        Err(err)=>format!("error reading file at {}: {}", file_path, err),
    }
}