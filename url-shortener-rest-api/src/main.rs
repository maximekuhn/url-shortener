use axum::{Json, Router, Server};
use axum::body::Body;
use axum::http::Request;
use axum::routing::get;
use serde::Deserialize;

#[derive(Deserialize)]
struct URLRequest {
    url: String,
}

async fn index(Json(payload): Json<URLRequest>) {
    println!("ENDPOINT: /");
    println!("body: {:?}", payload.url);
}

#[tokio::main]
async fn main() {
    let app =
        Router::new()
            .route("/", get(index));

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
