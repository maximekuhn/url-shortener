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
        let mut algorithms: HashMap<String, Box<dyn ShortenerAlgorithm + Send + Sync>> =
            HashMap::new();
        algorithms.insert("hash_md5".to_string(), Box::new(HashAlgorithm));

        Self { algorithms }
    }
}
