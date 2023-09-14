use crate::algorithms::{ErrorType, ShortenerAlgorithm};

/// A dummy URL shortener.
/// For now, it doesn't shorten anything.
/// The resolved URL is the input URL.
pub struct DummyShortener;

impl DummyShortener {
    pub fn new() -> Self {
        Self {}
    }
}

impl ShortenerAlgorithm for DummyShortener {

    /// The shortened URL is the input URL.
    /// Domain is not considered here.
    fn shorten(&mut self, _domain: String, url: String) -> Result<String, ErrorType> {
        Ok(url)
    }

    /// The shortened URL is the input URL.
    fn resolve(&mut self, shortened_url: String) -> Option<String> {
        Some(shortened_url)
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::dummy_shortener::DummyShortener;
    use crate::algorithms::ShortenerAlgorithm;

    #[test]
    fn shortened_url_should_be_the_input_url() {
        let mut sut = DummyShortener::new();

        let url = "https://www.rust-lang.org/";
        let shortened_url = sut.shorten("".to_string(), url.to_string()).unwrap();
        assert_eq!(url, &shortened_url);

        let resolved_url = sut.resolve(shortened_url).unwrap();
        assert_eq!(url, &resolved_url);
    }
}