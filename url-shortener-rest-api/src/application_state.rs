use std::sync::{Arc, Mutex};
use crate::algorithms_manager::AlgorithmsManager;
use crate::database_helper::DBHelper;

#[derive(Clone)]
pub struct ApplicationState {
    pub db_helper: Arc<Mutex<dyn DBHelper + Send + Sync>>,
    pub algorithms_manager: Arc<dyn AlgorithmsManager + Send + Sync>,
}