use std::collections::HashMap;
use crate::database_helper::DBHelper;

pub struct InMemoryDBHelper {
    mapping: HashMap<String, usize>,
}

impl DBHelper for InMemoryDBHelper {
    fn print(&self) {
        println!("{:#?}", self.mapping);
    }

    fn increment(&mut self) {
        match self.mapping.get(&"toto".to_string()) {
            None => self.mapping.insert("toto".to_string(), 1),
            Some(old) => self.mapping.insert("toto".to_string(), *old + 1),
        };
    }
}


impl InMemoryDBHelper {
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new()
        }
    }
}