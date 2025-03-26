use std::{fs, path::Path};

use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use serde_json::json;

pub async fn list_upload_dir() -> Result<Response, Response>{
    let mut files = Vec::new();
    let path = Path::new("uploads");

    if path.is_dir() {
        for entry in fs::read_dir(path).map_err(|_| {(StatusCode::INTERNAL_SERVER_ERROR, "error reading dir").into_response()})? {
            let entry = entry.map_err(|_| {(StatusCode::INTERNAL_SERVER_ERROR, "error listing dir").into_response()})?;
            let file_path = entry.path();

            if file_path.is_file() {
                if let Some(file_name) = file_path.file_name(){
                    files.push(file_name.to_string_lossy().to_string());
                }
            }
            
        }
    }
    Ok(
        Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json!({"avaliable_images": files}).to_string().into())
        .unwrap())
}