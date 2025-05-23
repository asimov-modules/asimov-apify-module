// This is free and unencumbered software released into the public domain.

#[derive(Clone, Debug, Default)]
pub struct Actor {
    pub id: &'static str,
    pub brand: &'static str,
    pub product: &'static str,
}
