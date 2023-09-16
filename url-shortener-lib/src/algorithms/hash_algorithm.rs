use crate::algorithms::ShortenerAlgorithm;

const CHARACTERS_TO_RETURN: usize = 5;
const ALGORITHM_NAME: &str = "hash_md5";

/// A simple hasher algorithm that computes the MD5 hash of the given URL
/// and returns the first 5 characters of it.
pub struct HashAlgorithm;

impl ShortenerAlgorithm for HashAlgorithm {
    fn shorten(&self, url: String) -> String {
        let digest = md5::compute(url);
        format!("{:x}", digest)[0..CHARACTERS_TO_RETURN].to_string()
    }

    fn get_name(&self) -> String {
        ALGORITHM_NAME.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::hash_algorithm::{HashAlgorithm, ALGORITHM_NAME};
    use crate::algorithms::ShortenerAlgorithm;

    #[test]
    fn it_should_correctly_shorten_url() {
        // expected MD5 hash: d75277cdffef995a46ae59bdaef1db86
        let url = "https://www.google.com/";

        let sut = HashAlgorithm;
        let expected = "d7527".to_string();
        let actual = sut.shorten(url.to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_should_return_correct_name() {
        let sut = HashAlgorithm;
        assert_eq!(ALGORITHM_NAME, sut.get_name());
    }
}
