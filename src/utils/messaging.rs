//! Traits that describe higher-order primitives over basic component messaging.

pub use uuid::Uuid;

pub trait MaybeRequest {
    fn request_id(&self) -> Option<Uuid>;
    fn response_id(&self) -> Option<Uuid>;
}

#[macro_export]
macro_rules! match_response {
    ($pat:pat $(if $guard:expr)? => $result:expr) => {
        |msg| match msg {
            $pat $(if $guard)? => Some($result),
            _ => None
        }
    };
}

#[macro_export]
macro_rules! expect_response {
    ($pat:pat $(if $guard:expr)? => $result:expr) => {
        |msg| match msg {
            $pat $(if $guard)? => Ok($result),
            _ => Err(anyhow::anyhow!("unexpected response message")),
        }
    };
}
