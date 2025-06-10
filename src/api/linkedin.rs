// This is free and unencumbered software released into the public domain.

#![allow(unused)]

use asimov_module::prelude::{FromStr, String, ToString};
use serde::Serialize;
use url::{ParseError, Url};

/// See: https://apify.com/apimaestro/linkedin-profile-detail
#[derive(Clone, Debug, Default, Serialize)]
pub struct LinkedInProfileScrapeRequest {
    pub username: String,
}

impl FromStr for LinkedInProfileScrapeRequest {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(input)?;
        let username = url
            .path_segments()
            .and_then(|mut segments| segments.nth(1)) // Skip "in" to get username
            .filter(|s| !s.is_empty())
            .ok_or(ParseError::InvalidDomainCharacter)?
            .to_string();
        // Reject extra segments (e.g., /in/username/extra)
        if url.path_segments().map(|segments| segments.count()).unwrap_or(0) > 2 {
            return Err(ParseError::InvalidDomainCharacter);
        }
        Ok(Self { username })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let request = LinkedInProfileScrapeRequest::from_str("https://www.linkedin.com/in/johndoe").unwrap();
        assert_eq!(request.username, "johndoe");
        assert!(LinkedInProfileScrapeRequest::from_str("https://www.linkedin.com/in/").is_err());
        assert!(LinkedInProfileScrapeRequest::from_str("https://www.linkedin.com/in/johndoe/extra").is_err());
        assert!(LinkedInProfileScrapeRequest::from_str("invalid").is_err());
    }
}