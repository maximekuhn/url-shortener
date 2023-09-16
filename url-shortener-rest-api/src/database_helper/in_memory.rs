use crate::database_helper::DBHelper;
use std::collections::HashMap;

pub struct InMemoryDBHelper {
    shortened_to_original: HashMap<String, String>,
}

impl DBHelper for InMemoryDBHelper {
    fn save(&mut self, original_url: String, shortened_url: String) {
        self.shortened_to_original
            .insert(shortened_url, original_url);
    }

    fn get_original_url(&self, shortened_url: String) -> Option<String> {
        match self.shortened_to_original.get(&shortened_url) {
            None => None,
            Some(url) => Some(url.clone()),
        }
    }
}

impl InMemoryDBHelper {
    pub fn new() -> Self {
        Self {
            shortened_to_original: HashMap::new(),
        }
    }
}
