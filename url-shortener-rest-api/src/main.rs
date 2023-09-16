use axum::{Json, Router, Server};
use axum::http::StatusCode;
use axum::routing::get;
use serde::Deserialize;
use url_shortener_lib::algorithms::hash_algorithm::HashAlgorithm;
use url_shortener_lib::algorithms::ShortenerAlgorithm;

#[derive(Deserialize)]
struct ShortenURLRequest {
    url: String,
}

async fn shorten(Json(payload): Json<ShortenURLRequest>) -> (StatusCode, Json<String>) {
    println!("ENDPOINT: /");
    let hash_algorithm = HashAlgorithm;
    let shortened_url = hash_algorithm.shorten(payload.url);
    (StatusCode::OK, Json(shortened_url))
}

#[tokio::main]
async fn main() {
    let app =
        Router::new()
            .route("/shorten", get(shorten));

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
