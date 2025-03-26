use axum::Router;
use axum::routing::{get, post};
use crate::handlers::greet_handler::say_hi;
use crate::handlers::say_name_handler::say_name;
use crate::handlers::get_image_handler::download_neko;
use crate::handlers::get_image_handler::get_img;
use crate::handlers::neko_api_handler::neko_api_img;
use crate::handlers::upload_handler::upload;
use crate::handlers::list_dir_handler::list_upload_dir;

pub fn create_router() -> Router {
    Router::new()
        .route("/greet", get(say_hi))
        .route("/say_name", post(say_name))
        .route("/random_img", get(download_neko))
        .route("/neko_img", post(neko_api_img))
        .route("/upload", post(upload))
        .route("/download/:filename", get(get_img))
        .route("/list_uploads", get(list_upload_dir))
}