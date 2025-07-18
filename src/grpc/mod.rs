use anyhow::Result;
use futures::Stream;
use std::pin::Pin;
use tonic::Status;

pub mod any_codec;
pub mod health;
pub mod json_codec;
pub mod msgpack_codec;

/// Commonly used type for gRPC server-streaming implementations
pub type SubscriptionStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;

#[rustfmt::skip]
pub mod service {
    include!("generated/json.architect.Health.rs");
    include!("generated/json.architect.Accounts.rs");
    include!("generated/json.architect.Auth.rs");
    include!("generated/json.architect.Core.rs");
    include!("generated/json.architect.Symbology.rs");
    include!("generated/json.architect.Marketdata.rs");
    include!("generated/json.architect.OptionsMarketdata.rs");
    include!("generated/json.architect.Orderflow.rs");
    include!("generated/json.architect.Oms.rs");
    include!("generated/json.architect.Folio.rs");
    include!("generated/json.architect.Algo.rs");
    include!("generated/json.architect.Cpty.rs");
    include!("generated/json.architect.Boss.rs");
}
