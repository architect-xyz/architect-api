//! This module contains the types we use to represent symbols and their metadata.
//!
//! There are two main concepts:
//!
//! - `Product`s, which are symbols that you can have positions in, e.g. currencies, stocks,
//!   futures, options, tokens, etc.
//! - `TradableProduct`s, which are pairs of base/quote products that you can trade in, e.g.
//!   BTC/USD, AAPL/USD, etc.
//!
//! Product names should be universally unique, and two products with the same name should
//! be considered fungible, e.g. 1 BTC is 1 BTC no matter where it's held.  For most
//! derivative products, they are not fungible with similar products from other exchanges,
//! so we discriminate them such as `BTC-USD BINANCE Perpetual` vs `BTC-USD OKX Perpetual`.
//!
//! Products such as options and event contracts often come in series, such as `AAPL US Option`
//! which comes in variety of strikes and expirations.  These can be referenced directly as
//! either an `OptionsSeries` or `EventContractSeries` respectively.
//!
//! ⚠️ All symbols, whether they are products, tradable products or series names should be
//! universally unique amongst each other, no matter the kind.
//!
//! Symbology is generally loaded from Architect, whether its Architect's centralized
//! symbology store, an Architect installation, or an Architect marketdata feed.
//!
//! Alternatively, symbol names can be constructed de novo from strings, if prior knowledge
//! of symbology and/or which symbols actually may exist is not required.
//!
//! ## Sending an order in a simple tradable product pair
//!
//! ```no_run
//! client.send_order(Order {
//!     tradable_product: "BTC Crypto/USD".into(),
//!     execution_venue: Some("BINANCE".into()),
//!     // ...
//! });
//! ```
//!
//! ## Sending an order for an option
//!
//! Suppose you have loaded options series information from a symbology service.
//!
//! ```no_run
//! let options_series_info: OptionsSeriesInfo =
//!     symbol_store.get_options_series_info("AAPL US Option".into())?;
//!
//! let tradable_option: TradableProduct =
//!     options_series_info.get_tradable_product(OptionsDimensions {
//!         expiration: "2025-01-17T00:00:00Z".parse().unwrap(),
//!         strike: dec!(150.0),
//!         put_call: PutCall::Call,
//!     })?;
//!
//! client.send_order(Order {
//!     tradable_product: tradable_option,
//!     execution_venue: None, // use Architect default routing
//!     // ...
//! });
//! ```
//!
//! ### De novo symbol construction
//!
//! Alternatively, without loading anything from a symbology service, you can
//! just guess at the tradable product name.
//!
//! ```no_run
//! client.send_order(Order {
//!     tradable_product: "APPL US 20241030 125.0 C Option".into(),
//!     // ...
//! })
//! ```

// TODO: make doc-comments compile
// TODO: marketdata info examples, execution info examples

pub mod event_contract_series;
pub mod metadata;
pub mod options_series;
pub mod product;
pub mod tradable_product;
pub mod venue;

pub use event_contract_series::*;
pub use metadata::*;
pub use options_series::*;
pub use product::*;
pub use tradable_product::*;
pub use venue::*;
