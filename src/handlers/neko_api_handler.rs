use mime_guess::from_path;
use reqwest::{self, StatusCode};
use axum::{body::{Body,Bytes}, http::Response, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive (Deserialize)]
struct NekoResponse{
    success: bool,
    message: String
}

#[derive(Deserialize, Serialize)]
pub struct InputBody{
    quest_type: String,
    sfw: bool
}


async  fn download_img(img_url: &str) -> Result<(Bytes, String), StatusCode> {
    match reqwest::get(img_url).await{
        Ok(response) => {
            let bytes = response.bytes().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let content_type = from_path(img_url)
                .first()
                .map(|m| m.to_string())
                .unwrap_or_else(|| "application/octet-stream".to_string());
            Ok((bytes, content_type))
        }
        Err(_) => Err(StatusCode::BAD_GATEWAY)
    }
}

pub async fn neko_api_img(Json(payload): Json<InputBody>) -> impl IntoResponse {
    if payload.quest_type.trim().is_empty(){
        return (StatusCode::BAD_REQUEST, "Empty quest type Valid quest_types:[neko, food, coffee]").into_response();
    }
    if payload.sfw {
        if !matches!(payload.quest_type.as_str(), | "neko" | "food" | "coffee"){
            return (StatusCode::BAD_REQUEST, "Not valid SFW quest_types:[neko, food, coffee]").into_response();
        }
    }
    let url = format!("https://nekobot.xyz/api/image?type={}", payload.quest_type);
    match reqwest::get(url).await {
        Ok(response) => match response.json::<NekoResponse>().await {
            Ok(body) => {
                if body.success{
                    match download_img(&body.message).await {
                        Ok((image_bytes, content_type)) => Response::builder()
                            .status(StatusCode::OK)
                            .header("Content-Type", content_type)
                            .body(Body::from(image_bytes))
                            .unwrap()
                            .into_response(),
                        Err(status) => (status, "Error al descargar la imagen").into_response(),
                    }
                }else{
                    let msg: String = format!("Neko APi returned {}", body.message);
                    (StatusCode::BAD_REQUEST, msg).into_response()
                }
            },
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al obtener el cuerpo").into_response(),
        },
        Err(_) => (StatusCode::BAD_GATEWAY, "Error al contactar la API").into_response(),
    }
}