use anyhow::Result;
use futures::Stream;
use std::pin::Pin;
use tonic::Status;

pub mod any_codec;
pub mod health;
pub mod json_codec;

/// Commonly used type for gRPC server-streaming implementations
pub type SubscriptionStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;

pub mod json_service {
    include!(concat!(env!("OUT_DIR"), "/json.architect.Health.rs"));
    include!(concat!(env!("OUT_DIR"), "/json.architect.Symbology.rs"));
    include!(concat!(env!("OUT_DIR"), "/json.architect.SymbologyV2.rs"));
    include!(concat!(env!("OUT_DIR"), "/json.architect.Marketdata.rs"));
}
