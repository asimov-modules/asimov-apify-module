// This is free and unencumbered software released into the public domain.

#![allow(unused)]

use asimov_module::prelude::{FromStr, String, Vec};
use serde::Serialize;

/// See: https://apify.com/kaitoeasyapi/premium-x-follower-scraper-following-data
#[derive(Clone, Debug, Default, Serialize)]
pub struct TwitterFollowingScrapeRequest {
    #[serde(rename = "getFollowers")]
    pub get_followers: bool,

    #[serde(rename = "getFollowing")]
    pub get_following: bool,

    #[serde(rename = "maxFollowers")]
    pub max_followers: u32,

    #[serde(rename = "maxFollowings")]
    pub max_followings: u32,

    pub user_names: Vec<String>,
}

impl FromStr for TwitterFollowingScrapeRequest {
    type Err = url::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        url::Url::parse(input).map(|url| match url.path_segments().and_then(|mut s| s.next()) {
            None => Self::default(),
            Some(path) => Self {
                user_names: Vec::from([path.into()]),
                get_followers: true,
                get_following: true,
                max_followers: 200,
                max_followings: 200,
                ..Default::default()
            },
        })
    }
}
