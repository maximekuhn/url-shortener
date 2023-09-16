pub mod in_memory;

pub trait DBHelper: Send + Sync {
    fn save(&mut self, original_url: String, shortened_url: String);

    fn get_original_url(&self, shortened_url: String) -> Option<String>;
}
