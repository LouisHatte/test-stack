use axum::{Json, Router, routing::get};
use serde::Serialize;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[derive(Serialize)]
struct Message {
    text: String,
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route(
            "/hello",
            get(|| async {
                Json(Message {
                    text: "Hello from Rust with CORS!".into(),
                })
            }),
        )
        .layer(cors);

    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();

    println!("Server running at http://127.0.0.1:3001");

    axum::serve(listener, app).await.unwrap();
}
