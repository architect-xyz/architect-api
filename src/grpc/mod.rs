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

macro_rules! include_generated {
    ($file:literal) => {
        #[cfg(feature = "docs-rs")]
        include!(concat!(env!("OUT_DIR"), "/", $file));

        #[cfg(not(feature = "docs-rs"))]
        include!(concat!("generated/", $file));
    };
}
#[rustfmt::skip]
pub mod service {

    include_generated!("json.architect.Health.rs");
    include_generated!("json.architect.Accounts.rs");
    include_generated!("json.architect.Auth.rs");
    include_generated!("json.architect.Core.rs");
    include_generated!("json.architect.Symbology.rs");
    include_generated!("json.architect.Marketdata.rs");
    include_generated!("json.architect.OptionsMarketdata.rs");
    include_generated!("json.architect.Orderflow.rs");
    include_generated!("json.architect.Oms.rs");
    include_generated!("json.architect.Folio.rs");
    include_generated!("json.architect.Algo.rs");
    include_generated!("json.architect.Cpty.rs");
    include_generated!("json.architect.Boss.rs");
}
