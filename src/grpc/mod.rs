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
    include!("generated/architect.Health.rs");
    include!("generated/architect.Accounts.rs");
    include!("generated/architect.Auth.rs");
    include!("generated/architect.Core.rs");
    include!("generated/architect.Symbology.rs");
    include!("generated/architect.Marketdata.rs");
    include!("generated/architect.OptionsMarketdata.rs");
    include!("generated/architect.Orderflow.rs");
    include!("generated/architect.Oms.rs");
    include!("generated/architect.Folio.rs");
    include!("generated/architect.Algo.rs");
    include!("generated/architect.Cpty.rs");
    include!("generated/architect.Boss.rs");
}
