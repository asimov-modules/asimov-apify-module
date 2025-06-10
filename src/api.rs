// This is free and unencumbered software released into the public domain.

use core::error::Error;

use asimov_module::prelude::{Box, format, Result, String};
pub use asimov_module::secrecy::{ExposeSecret, SecretString};
use serde::Serialize;

pub use google::*;
pub use instagram::*;
pub use linkedin::*;
pub use twitter::*;

mod google;
mod twitter;
mod linkedin;
mod instagram;

const GOOGLE_SEARCH_ACTOR: &str = "apify~google-search-scraper";
const X_FOLLOWS_ACTOR: &str = "C2Wk3I6xAqC4Xi63f";
const LINKEDIN_PROFILE_ACTOR: &str = "VhxlqQXRwhW8H5hNV";
const INSTAGRAM_PROFILE_ACTOR: &str = "dSCLg0C3YEZ83HzYX";

/// See: https://docs.apify.com/api
#[derive(Debug)]
pub struct Apify {
    pub(crate) api_key: SecretString,
}

impl Apify {
    /// Creates a new Apify client with the provided API key
    /// Returns an error if the API key is empty
    pub fn new(api_key: SecretString) -> Result<Self, Box<dyn Error>> {
        if api_key.expose_secret().is_empty() {
            return Err("Invalid API key".into());
        }
        Ok(Self { api_key })
    }

    /// Makes a POST request to the Apify API
    fn make_request<T: Serialize>(
        &self,
        actor: &str,
        request: &T,
    ) -> Result<String, Box<dyn Error>> {
        // See: https://docs.apify.com/academy/api/run-actor-and-retrieve-data-via-api
        // See: https://docs.apify.com/api/v2/act-run-sync-post
        let url = format!("https://api.apify.com/v2/acts/{}/run-sync-get-dataset-items", actor);
        let mut response = ureq::post(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.api_key.expose_secret()),
            )
            .send_json(request)?;
        let response_body = response.body_mut().read_to_string()?;
        Ok(response_body)
    }

    /// Performs a Google search scrape
    /// See: https://apify.com/apify/google-search-scraper
    pub fn google_search(&self, request: &GoogleSearchRequest) -> Result<String, Box<dyn Error>> {
        self.make_request(GOOGLE_SEARCH_ACTOR, request)
    }

    /// Performs an X/Twitter followers/followings scrape
    /// See: https://apify.com/kaitoeasyapi/premium-x-follower-scraper-following-data
    pub fn x_follows(
        &self,
        request: &TwitterFollowingScrapeRequest,
    ) -> Result<String, Box<dyn Error>> {
        self.make_request(X_FOLLOWS_ACTOR, request)
    }

    /// Performs a LinkedIn profile scrape
    /// See: https://apify.com/apimaestro/linkedin-profile-detail
    pub fn linkedin_profile(
        &self,
        request: &LinkedInProfileScrapeRequest,
    ) -> Result<String, Box<dyn Error>> {
        self.make_request(LINKEDIN_PROFILE_ACTOR, request)
    }

    /// Performs an Instagram profile scrape
    /// See: https://apify.com/apify/instagram-profile-scraper
    pub fn instagram_profile(
        &self,
        request: &InstagramProfileScrapeRequest,
    ) -> Result<String, Box<dyn Error>> {
        self.make_request(INSTAGRAM_PROFILE_ACTOR, request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_api_key() {
        let result = Apify::new(SecretString::from(""));
        assert!(result.is_err());
    }
}
