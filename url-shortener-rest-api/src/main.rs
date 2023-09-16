use std::error::Error;
use std::sync::{Arc, Mutex};
use axum::{Router, Server, ServiceExt};
use axum::routing::{get, post};
use crate::algorithms_manager::AlgorithmsManager;
use crate::application_state::ApplicationState;
use crate::database_helper::DBHelper;
use crate::database_helper::in_memory::InMemoryDBHelper;

mod application_state;
mod database_helper;
mod algorithms_manager;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create the application state
    let application_state = create_application_state();

    // Create router
    let app = Router::new()
        .route("/api/v1/shorten", post(routes::shorten))
        .route("/api/v1/resolve", get(routes::resolve))
        .with_state(application_state);

    // Run the server
    Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn create_application_state() -> ApplicationState {
    ApplicationState {
        db_helper: Arc::new(Mutex::new(InMemoryDBHelper::new()))
    }
}
