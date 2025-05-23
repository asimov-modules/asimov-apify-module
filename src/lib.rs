// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

pub mod api;

mod actor;
mod actors;

use actor::Actor;

pub fn find_actor_for(url: impl AsRef<str>) -> Option<&'static Actor> {
    let url = url.as_ref();
    for (url_pattern, actor) in actors::URL_PREFIX_TO_ACTOR.iter().rev() {
        if url.starts_with(url_pattern) {
            return Some(actor);
        }
    }
    None // not found
}
