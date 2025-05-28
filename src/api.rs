// This is free and unencumbered software released into the public domain.

mod google;
pub use google::*;

mod twitter;
pub use twitter::*;

pub use asimov_module::secrecy::{ExposeSecret, SecretString};

use asimov_module::prelude::{Box, Result, String, format};
use core::error::Error;

/// See: https://docs.apify.com/api
#[derive(Debug)]
pub struct Apify {
    pub(crate) api_key: SecretString,
}

impl Apify {
    pub fn new(api_key: SecretString) -> Self {
        Self { api_key }
    }

    /// See: https://apify.com/apify/google-search-scraper
    pub fn google_search(&self, request: &GoogleSearchRequest) -> Result<String, Box<dyn Error>> {
        let actor_url = format!(
            "https://api.apify.com/v2/acts/{}/run-sync-get-dataset-items",
            "apify~google-search-scraper"
        );
        let mut response = ureq::post(actor_url)
            .header(
                "Authorization",
                format!("Bearer {}", self.api_key.expose_secret()),
            )
            .send_json(request)?;
        let response_body = response.body_mut().read_to_string()?;
        Ok(response_body)
    }

    /// See: https://apify.com/kaitoeasyapi/premium-x-follower-scraper-following-data
    pub fn x_follows(
        &self,
        request: &TwitterFollowingScrapeRequest,
    ) -> Result<String, Box<dyn Error>> {
        // See: https://docs.apify.com/academy/api/run-actor-and-retrieve-data-via-api
        // See: https://docs.apify.com/api/v2/act-run-sync-post
        let actor_url = format!(
            "https://api.apify.com/v2/acts/{}/run-sync-get-dataset-items",
            "C2Wk3I6xAqC4Xi63f"
        );
        let mut response = ureq::post(actor_url)
            .header(
                "Authorization",
                format!("Bearer {}", self.api_key.expose_secret()),
            )
            .send_json(request)?;
        let response_body = response.body_mut().read_to_string()?;
        Ok(response_body)
    }
}
