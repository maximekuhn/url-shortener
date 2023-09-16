use crate::application_state::ApplicationState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

pub async fn shorten(
    State(state): State<ApplicationState>,
    Json(payload): Json<URLShortenRequest>,
) -> (StatusCode, Json<URLShortenResponse>) {
    println!("📞️ /api/v1/shorten");
    let original_url = payload.url;
    let algorithm_name = payload.algorithm_name.unwrap_or("hash_md5".to_string());
    let shortened_url = state
        .algorithms_manager
        .shorten_with_algorithm(algorithm_name, original_url.clone());
    return match shortened_url {
        None => {
            println!("Could not find algorithm to shorten the given URL");
            (
                StatusCode::NO_CONTENT,
                Json(URLShortenResponse {
                    shortened_url: "".to_string(),
                }),
            )
        }
        Some(url) => {
            println!("Shortened URL: {}", url);
            state
                .db_helper
                .lock()
                .expect("Poisoned mutex")
                .save(original_url, url.clone());
            (
                StatusCode::CREATED,
                Json(URLShortenResponse { shortened_url: url }),
            )
        }
    };
}

pub async fn resolve(
    State(state): State<ApplicationState>,
    Json(payload): Json<URLResolveRequest>,
) -> (StatusCode, Json<URLResolveResponse>) {
    println!("📞️ /api/v1/resolve");
    let shortened_url = payload.shortened_url;
    return match state
        .db_helper
        .lock()
        .expect("poisoned mutex")
        .get_original_url(shortened_url)
    {
        None => {
            println!("Could not find original URL");
            (
                StatusCode::NOT_FOUND,
                Json(URLResolveResponse {
                    original_url: "".to_string(),
                }),
            )
        }
        Some(original_url) => {
            println!("Original URL: {}", original_url);
            (
                StatusCode::OK,
                Json(URLResolveResponse {
                    original_url: original_url,
                }),
            )
        }
    };
}

// Requests
#[derive(Deserialize)]
pub struct URLShortenRequest {
    pub url: String,
    pub algorithm_name: Option<String>,
}

#[derive(Deserialize)]
pub struct URLResolveRequest {
    pub shortened_url: String,
}

// Responses
#[derive(Serialize)]
pub struct URLShortenResponse {
    pub shortened_url: String,
}

#[derive(Serialize)]
pub struct URLResolveResponse {
    pub original_url: String,
}
