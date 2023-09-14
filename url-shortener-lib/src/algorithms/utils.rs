// All terms in this file are based on :
// https://developer.mozilla.org/en-US/docs/Learn/Common_questions/Web_mechanics/What_is_a_URL


const SCHEME_SEPARATOR: &str = "://";
fn remove_scheme(url: &str) -> &str {
    if url.contains(SCHEME_SEPARATOR) {
        let url_parts: Vec<&str> = url.split(SCHEME_SEPARATOR).collect();
        return match url_parts.len() == 2 {
            true => url_parts.get(1).unwrap(),
            false => url,
        };
    }
    url
}

fn get_tld(url: &str) -> &str {
    let without_scheme = remove_scheme(url);
    let without_location = {
        match without_scheme.contains("/") {
            true => *without_scheme.split("/").collect::<Vec<&str>>().first().unwrap(),
            false => without_scheme,
        }
    };
    let domains: Vec<&str> = without_location.split(".").collect();
    *domains.last().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::algorithms::utils::{get_tld, remove_scheme};

    #[test]
    fn it_should_remove_scheme() {
        const AUTHORITY: &str = "www.example.com";
        let url1 = format!("http://{}", AUTHORITY);
        let url2 = format!("file://{}", AUTHORITY);
        let url3 = format!("protocol://{}", AUTHORITY);

        assert_eq!(AUTHORITY, remove_scheme(&url1));
        assert_eq!(AUTHORITY, remove_scheme(&url3));
        assert_eq!(AUTHORITY, remove_scheme(&url2));
    }

    #[test]
    fn no_scheme_to_remove() {
        const AUTHORITY: &str = "www.apple.com/mac";
        let url1 = AUTHORITY.to_string();
        assert_eq!(AUTHORITY, remove_scheme(&url1));
    }

    #[test]
    fn it_should_return_tld() {
        let url1 = "http://www.github.com/api/v1";
        let url2 = "http://www.aaaaaa.org";
        let url3 = "www.ooooooo.fr/api/v1";
        let url4 = "www.ooooooo.fr/api/v1/users/all";

        assert_eq!("com", get_tld(url1));
        assert_eq!("org", get_tld(url2));
        assert_eq!("fr", get_tld(url3));
        assert_eq!("fr", get_tld(url4));
    }
}