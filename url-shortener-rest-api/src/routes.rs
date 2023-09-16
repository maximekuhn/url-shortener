use crate::application_state::ApplicationState;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;

pub async fn shorten(
    State(state): State<ApplicationState>,
    Json(payload): Json<URLShortenRequest>,
) {
    println!("📞️ /api/v1/shorten");
    let original_url = payload.url;
    let algorithm_name = payload.algorithm_name.unwrap_or("hash_md5".to_string());
    let shortened_url = state
        .algorithms_manager
        .shorten_with_algorithm(algorithm_name, original_url.clone());
    match shortened_url {
        None => {
            println!("Could not find algorithm to shorten the given URL");
        }
        Some(url) => {
            println!("Shortened URL: {}", url);
            state
                .db_helper
                .lock()
                .expect("Poisoned mutex")
                .save(original_url, url);
        }
    };
}

pub async fn resolve(
    State(state): State<ApplicationState>,
    Json(payload): Json<URLResolveRequest>,
) {
    println!("📞️ /api/v1/resolve");
    let shortened_url = payload.shortened_url;
    match state
        .db_helper
        .lock()
        .expect("poisoned mutex")
        .get_original_url(shortened_url)
    {
        None => {
            println!("Could not find original URL");
        }
        Some(original_url) => {
            println!("Original URL: {}", original_url);
        }
    };
}

#[derive(Deserialize)]
pub struct URLShortenRequest {
    pub url: String,
    pub algorithm_name: Option<String>,
}

#[derive(Deserialize)]
pub struct URLResolveRequest {
    pub shortened_url: String,
}
