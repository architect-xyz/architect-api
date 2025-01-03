//! # Required dependencies
//!
//! ```toml
//! [dependencies]
//! tonic = <tonic-version>
//!
//! [build-dependencies]
//! tonic-build = <tonic-version>
//! ```
//!

#![recursion_limit = "256"]
#![warn(rust_2018_idioms, unreachable_pub)]

pub mod manual;
mod server;

mod code_gen;
pub mod code_gen_types;
pub use code_gen::CodeGenBuilder;
