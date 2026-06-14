use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct FileParams{
    pub file:String,
}

pub async fn read_file_safe(Query(params):Query<FileParams>)-> impl IntoResponse{
    let base_dir = Path::new("templates");
    let canonical_base = match base_dir.canonicalize(){
        Ok(path) => path,
        Err(err)=>return
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Server configuration error: {}", err)).into_response(),
    };

    let target_path = base_dir.join(&params.file);
    let canonical_target = match target_path.canonicalize(){
        Ok(path)=>path,
        Err(_)=>return 
        (StatusCode::NOT_FOUND, "file not found or invalid".to_string()).into_response(),
    };

    if !canonical_target.starts_with(&canonical_base){
        return (StatusCode::FORBIDDEN, "Access denied : path traversal detected".to_string()).into_response();
    
    }
    match fs::read_to_string(&canonical_target){
        Ok(content)=>(StatusCode::OK, content).into_response(),
        Err(err)=>(StatusCode::INTERNAL_SERVER_ERROR, format!("Error reading file: {}",err)).into_response(),
    }   
    
}