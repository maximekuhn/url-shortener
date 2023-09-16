use axum::extract::State;
use crate::algorithms_manager::AlgorithmsManager;
use crate::application_state::ApplicationState;
use crate::database_helper::DBHelper;

pub async fn shorten(State(state) : State<ApplicationState>) {
    println!("shorten");
    let mut db_helper = state.db_helper.lock().expect("poisoned");
    db_helper.increment();
    db_helper.print();
}

pub async fn resolve(State(state) : State<ApplicationState>) {
    println!("resolve");
    let mut db_helper = state.db_helper.lock().expect("poisoned");
    db_helper.increment();
    db_helper.print();
}
