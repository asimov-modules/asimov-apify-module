use asimov_module::prelude::{FromStr, String, ToString, Vec};
use serde::Serialize;
use std::vec;
use url::{ParseError, Url};

/// See: https://apify.com/apify/instagram-profile-scraper
#[derive(Clone, Debug, Default, Serialize)]
pub struct InstagramProfileScrapeRequest {
    pub usernames: Vec<String>,
}

impl FromStr for InstagramProfileScrapeRequest {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(input)?;

        let username = url
            .path_segments()
            .and_then(|mut segments| segments.next())
            .filter(|s| !s.is_empty())
            .ok_or(ParseError::InvalidDomainCharacter)?
            .to_string();

        Ok(Self {
            usernames: vec![username],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let request =
            InstagramProfileScrapeRequest::from_str("https://www.instagram.com/humansofny/")
                .unwrap();
        assert_eq!(request.usernames[0], "humansofny");
        assert!(InstagramProfileScrapeRequest::from_str("https://www.instagram.com/").is_err());
        assert!(InstagramProfileScrapeRequest::from_str("invalid").is_err());
    }
}
