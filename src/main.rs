mod routes;
mod handlers;

#[tokio::main]
async fn main() {
    let app = routes::routes::create_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running!");
    axum::serve(listener, app).await.unwrap();
}
