use std::fs;
use std::path::Path;
use axum::{http::{header, StatusCode}, response::{IntoResponse, Response}, extract::Path as Pth};
use rand::seq::IndexedRandom;
use tokio::fs as async_fs;
use mime_guess;


async fn download_img(file_path: String) ->  Response {
    if !Path::new(&file_path).exists() {
        return (StatusCode::NOT_FOUND, "File not found!").into_response();
    }

    match async_fs::read(&file_path).await {
        Ok(contents) => {
            let extension = mime_guess::from_path(&file_path).first_or_octet_stream();
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, extension.as_ref())],
                contents,
            ).into_response()
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error at reading the file").into_response(),
    }

}

fn get_random_img_path(dir: &str) -> Result<Option<String>, std::io::Error> {

    let mut imgs = Vec::new();
    let path = Path::new(dir);

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();

            if file_path.is_file() {
                if let Some(ext) = file_path.extension() {
                    if ["jpg", "jpeg", "png", "gif", "bmp", "webp"].contains(&ext.to_str().unwrap_or("")) {
                        imgs.push(file_path.to_string_lossy().into_owned());
                    }
                }
            }
            
        }
    }

    Ok(imgs.choose(&mut rand::rng()).cloned())
}



pub async fn download_neko() -> impl IntoResponse {

    let file_path = match get_random_img_path("nekos") {
        Ok(Some(path)) => path,
        Ok(None) => return (StatusCode::NOT_FOUND, "No images found!").into_response(),
        Err(_) => return (StatusCode::NOT_FOUND, "No images found!").into_response()
    };
    
    download_img(file_path).await
    
}

pub async fn get_img(Pth(file_name): Pth<String>) -> impl IntoResponse {
    let file_path: String = format!("uploads/{}",file_name);
    println!("{}", file_path);
    download_img(file_path).await
}

