use anyhow::Result;
use futures::Stream;
use std::pin::Pin;
use tonic::Status;

pub mod any_codec;
pub mod health;
pub mod json_codec;

/// Commonly used type for gRPC server-streaming implementations
pub type SubscriptionStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;

// If adding new services, ensure you also add reference to `.sdk.rs` file in `./codegen.rs`
#[rustfmt::skip]
pub mod json_service {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/schema/generated/json.architect.Health.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/schema/generated/json.architect.Symbology.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/schema/generated/json.architect.Marketdata.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/schema/generated/json.architect.MarketdataSnapshots.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/schema/generated/json.architect.Orderflow.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/schema/generated/json.architect.Folio.rs"));
}
