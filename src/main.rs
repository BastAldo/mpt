use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

mod handlers;
mod models;

#[tokio::main]
async fn main() {
    // Define a permissive CORS layer for development.
    // This allows our future WASM frontend to make requests to this API.
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create the main router for our application.
    let app = Router::new()
        .route("/api/v1/workouts/:date", get(handlers::get_workout_by_date))
        .layer(cors);

    // Define the server address.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("->> LISTENING on http://{}", listener.local_addr().unwrap());

    // Start the server.
    axum::serve(listener, app).await.unwrap();
}