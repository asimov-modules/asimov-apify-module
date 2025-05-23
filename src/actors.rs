// This is free and unencumbered software released into the public domain.

use crate::actor::Actor;

pub static URL_PREFIX_TO_ACTOR: [(&str, Actor); 2] = [
    // See: https://apify.com/apify/google-search-scraper
    (
        "https://www.google.com/search?q=",
        Actor {
            id: "apify~google-search-scraper",
            brand: "Apify",
            product: "Google Search Results Scraper",
        },
    ),
    // See: https://apify.com/kaitoeasyapi/premium-x-follower-scraper-following-data
    (
        "https://x.com/", // TODO: https://x.com/:account/following
        Actor {
            id: "C2Wk3I6xAqC4Xi63f",
            brand: "Kaito Easy API",
            product: "Twitter (X) Follower Scraper",
        },
    ),
];
