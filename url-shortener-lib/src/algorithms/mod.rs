pub mod hash_algorithm;

/// A trait that all algorithms must implement.
pub trait ShortenerAlgorithm {

    /// Shorten the given URL and returns the shortened URL.
    fn shorten(&self, url: String) -> String;
}