pub mod dummy_shortener;
mod utils;

/// Errors that can be returned when calling
/// `shorten` method on a `ShortenerAlgorithm` implementation.
#[derive(Debug)]
pub enum ErrorType {
    /// The given URL can't be shortened because there
    /// is a collision.
    Collision,
}

/// A trait, that all shortener algorithms must implement.
pub trait ShortenerAlgorithm {

    /// Shorten the given URL and add the domain in front of it.
    fn shorten(&mut self, domain: String, url: String) -> Result<String, ErrorType>;

    /// Tries to resolve a shortened URL.
    /// Returns None if the shortened URL can't be resolved, the result otherwise.
    fn resolve(&mut self, shortened_url: String) -> Option<String>;

    /// Returns the algorithm name
    fn get_name(&self) -> String;
}