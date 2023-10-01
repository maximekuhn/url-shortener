use axum::{
    extract::{State, Path},
    http::StatusCode,
    routing::{get, post},
    Json, Router, Server,
};
use serde::Deserialize;
use tower_http::cors::CorsLayer;
use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, RwLock},
};
use url_shortener_lib::algorithms::ShortenerAlgorithm;

trait DBTrait {
    fn save(&mut self, original_url: String, shortened_url: String);

    fn get(&self, shortened_url: &str) -> Option<String>;
}

struct InMemoryDB {
    store: HashMap<String, String>,
}

impl InMemoryDB {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

impl DBTrait for InMemoryDB {
    fn save(&mut self, original_url: String, shortened_url: String) {
        self.store.insert(shortened_url, original_url);
    }

    fn get(&self, shortened_url: &str) -> Option<String> {
        match self.store.get(shortened_url) {
            Some(original_url) => Some(original_url.clone()),
            None => None,
        }
    }
}

type DB = Arc<RwLock<dyn DBTrait + Send + Sync>>;

#[derive(Clone)]
struct AppState {
    db: DB,
}

#[derive(Deserialize)]
struct ShortenUrl {
    original_url: String,
}


async fn shorten(
    State(state): State<AppState>,
    Json(payload): Json<ShortenUrl>,
) -> (StatusCode, Json<String>) {
    println!("->> /shorten");
    let db = &mut state.db.write().expect("Lock poisoned");
    let original_url = payload.original_url;
    let algorithm = url_shortener_lib::algorithms::hash_algorithm::HashAlgorithm;
    let shortened_url = algorithm.shorten(original_url.clone());
    db.save(original_url, shortened_url.clone());
    (StatusCode::OK, Json(shortened_url))
}

async fn resolve(
    State(state): State<AppState>,
    Path(shortened_url): Path<String>,
) -> (StatusCode, Json<String>) {
    println!("->> /resolve");
    let db = state.db.read().expect("Lock poisoned");
    match db.get(&shortened_url) {
        Some(original_url) => (StatusCode::OK, Json(original_url)),
        None => (StatusCode::NOT_FOUND, Json("".to_string())),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create state
    let app_state = AppState {
        db: Arc::new(RwLock::new(InMemoryDB::new())),
    };

    // Create router
    let router = Router::new()
        .route("/shorten", post(shorten))
        .route("/resolve/:shortened_url", get(resolve))
        .with_state(app_state)
        .layer(CorsLayer::permissive());

    // Start the server, listening on all interfaces, port 9090
    Server::bind(&"0.0.0.0:9090".parse()?)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
