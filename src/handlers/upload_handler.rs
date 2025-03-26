use std::path::Path;
use axum::{extract:: Multipart, response::{IntoResponse, Response}};
use mime_guess::from_path;
use serde_json::json;
use tokio::{fs::File, io::AsyncWriteExt};
use reqwest::StatusCode;
use uuid::Uuid;



pub async fn upload(mut multipart: Multipart) -> Result<Response, Response> {

    let mut saved_files:Vec<String> = Vec::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| {(StatusCode::PAYLOAD_TOO_LARGE, "Files are bigger than 2MB").into_response()})?
    {
        if let Some("files") = field.name(){
            
            if let Some(file_name) = field.file_name(){

                let extension = from_path(file_name)
                    .first_raw()
                    .and_then(|mime| mime.split('/').nth(1))
                    .unwrap_or("");


                let file_name =  if extension.is_empty(){
                    Uuid::new_v4().to_string()
                }else{
                    format!("{}.{}", Uuid::new_v4(), extension)
                };


                let upload_path = format!("uploads/{}", file_name);



                if !Path::new("uploads").exists(){
                    tokio::fs::create_dir("uploads").await.map_err(|_| {
                        (StatusCode::INTERNAL_SERVER_ERROR, "Error creating uploads directory!").into_response()
                    })?;
                }
                
                let bytes = field.bytes().await.map_err(|_| {
                    (StatusCode::PAYLOAD_TOO_LARGE, "Files are bigger than 2MB").into_response()
                })?;
                
                let mut saved_file = File::create(&upload_path).await.map_err(|_| {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Error saving the file!").into_response()
                })?;
                
                saved_file.write(&bytes).await.map_err(|_|{
                    (StatusCode::INTERNAL_SERVER_ERROR, "Error writing the file!").into_response()
                })?;

                println!("File {} saved!", upload_path);
                saved_files.push(file_name);    
            }
            
        }
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json!({"saved_files": saved_files}).to_string().into())
        .unwrap())

}