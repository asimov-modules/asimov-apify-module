// This is free and unencumbered software released into the public domain.

use crate::actor::Actor;

pub static URL_PREFIX_TO_ACTOR: [(&str, Actor); 4] = [
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
    // See: https://apify.com/apimaestro/linkedin-profile-detail
    (
        "https://www.linkedin.com/in/",
        Actor {
            id: "VhxlqQXRwhW8H5hNV",
            brand: "API Maestro",
            product: "Linkedin Profile Details Scraper (No Cookies Required)",
        },
    ),
    // See: https://apify.com/apify/instagram-profile-scraper
    (
        "https://www.instagram.com/",
        Actor {
            id: "dSCLg0C3YEZ83HzYX",
            brand: "Apify",
            product: "Instagram Profile Scraper",
        },
    ),
];
