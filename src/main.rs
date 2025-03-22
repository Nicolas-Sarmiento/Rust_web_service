use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let address = SocketAddr::from(([0,0, 0, 0], 8000));
    println!("Server running on http://{}", address);
    
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async  fn handler() -> &'static str {
    return "Hello Server!\n";
}