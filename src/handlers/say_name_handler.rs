use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Name {
    name: String,
}

pub async fn say_name(Json(payload): Json<Name>) -> impl IntoResponse {
    Json(serde_json::json!({ "message": format!("Hola, {}!", payload.name) }))
}