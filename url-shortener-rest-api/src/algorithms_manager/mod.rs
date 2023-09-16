pub mod production_algorithm_manager;

pub trait AlgorithmsManager: Send + Sync {
    fn shorten_with_algorithm(&self, algorithm_name: String, url: String) -> Option<String>;
}
