use crate::algorithms_manager::AlgorithmsManager;
use std::collections::HashMap;
use url_shortener_lib::algorithms::hash_algorithm::HashAlgorithm;
use url_shortener_lib::algorithms::ShortenerAlgorithm;

pub struct ProductionAlgorithmManager {
    algorithms: HashMap<String, Box<dyn ShortenerAlgorithm + Send + Sync>>,
}

impl AlgorithmsManager for ProductionAlgorithmManager {
    fn shorten_with_algorithm(&self, algorithm_name: String, url: String) -> Option<String> {
        match self.algorithms.get(&algorithm_name) {
            None => None,
            Some(algorithm) => Some(algorithm.shorten(url)),
        }
    }
}

impl ProductionAlgorithmManager {
    pub fn new() -> Self {
        Self {
            algorithms: HashMap::new(),
        }
    }

    pub fn with_hash_algorithm(mut self) -> Self {
        let hash_algorithm = HashAlgorithm;
        self.algorithms
            .insert(hash_algorithm.get_name(), Box::new(hash_algorithm));
        self
    }
}
