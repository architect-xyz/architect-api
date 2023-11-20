//! Traits that describe higher-order primitives over basic component messaging.

pub use uuid::Uuid;

pub trait MaybeRequest {
    fn request_id(&self) -> Option<Uuid>;
    fn response_id(&self) -> Option<Uuid>;
}
