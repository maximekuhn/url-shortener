use std::sync::{Arc, Mutex};
use crate::database_helper::DBHelper;

#[derive(Clone)]
pub struct ApplicationState {
    pub db_helper: Arc<Mutex<dyn DBHelper + Send + Sync>>,
}