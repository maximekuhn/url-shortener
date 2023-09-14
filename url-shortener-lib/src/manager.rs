use std::collections::HashMap;
use crate::algorithms::dummy_shortener::DummyShortener;
use crate::algorithms::{ErrorType, ShortenerAlgorithm};

enum ManagerError {
    AlgorithmDoesNotExist,
    AlgorithmIsUnableToResolveShortenedURL,
    AlgorithmError(ErrorType),
}

pub struct Manager {
    algorithms: HashMap<String, Box<dyn ShortenerAlgorithm>>
}

impl Manager {
    pub fn new() -> Self {
        let mut manager = Self {
            algorithms: HashMap::new(),
        };
        manager.register_all_algorithms();
        manager
    }

    fn register_all_algorithms(&mut self) {
        let dummy = DummyShortener::new();
        self.algorithms.insert(dummy.get_name(), Box::new(dummy));
    }

    fn shorten(&mut self, algo_name: String, domain: String, url: String) -> Result<String, ManagerError> {
        match self.algorithms.get_mut(&algo_name) {
            None => Err(ManagerError::AlgorithmDoesNotExist),
            Some(algorithm) => {
                return match algorithm.shorten(domain, url) {
                    Ok(shortened_url) => Ok(shortened_url),
                    Err(e) => Err(ManagerError::AlgorithmError(e)),
                };
            },
        }
    }

    fn resolve(&mut self, algo_name: String, shortened_url: String) -> Result<String, ManagerError> {
        match self.algorithms.get_mut(&algo_name) {
            None => Err(ManagerError::AlgorithmDoesNotExist),
            Some(algorithm) => {
                return match algorithm.resolve(shortened_url) {
                    None => Err(ManagerError::AlgorithmIsUnableToResolveShortenedURL),
                    Some(resolved_url) => Ok(resolved_url),
                };
            }
        }
    }
}