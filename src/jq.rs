// This is free and unencumbered software released into the public domain.

pub use ::jq::*;

#[cfg(feature = "std")]
pub fn google_search() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/google_search.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn google_search() -> JsonFilter {
    include_str!("jq/google_search.jq").parse().unwrap()
}
