use axum::Router;
use axum::routing::{get, post};
use crate::handlers::greet_handler::say_hi;
use crate::handlers::say_name_handler::say_name;
use crate::handlers::get_image_handler::download_img;
use crate::handlers::neko_api_handler::neko_api_img;

pub fn create_router() -> Router {
    Router::new()
        .route("/greet", get(say_hi))
        .route("/say_name", post(say_name))
        .route("/random_img", get(download_img))
        .route("/neko_img", post(neko_api_img))
}