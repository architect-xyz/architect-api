//! This module provides utilities for generating `tonic` service definitions for use by our client
//! sdk code generators.
//!
//! [2024-12-30] dkasten: fork of tonic-build/src/manual.rs at
//! https://github.com/hyperium/tonic/commit/1c5150aaf62d6e72ce6c07966a9f19ceedb52702
//!
//! # Example
//!
//! ```rust,ignore
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let greeter_service = tonic_build::manual::Service::builder()
//!         .name("Greeter")
//!         .package("helloworld")
//!         .method(
//!             tonic_build::manual::Method::builder()
//!                 .name("say_hello")
//!                 .route_name("SayHello")
//!                 // Provide the path to the Request type
//!                 .input_type("crate::HelloRequest")
//!                 // Provide the path to the Response type
//!                 .output_type("super::HelloResponse")
//!                 // Provide the path to the Codec to use
//!                 .codec_path("crate::JsonCodec")
//!                 .build(),
//!         )
//!         .build();
//!
//!     // note we run first with a borrowed reference since tonic takes ownership
//!     sdk_build::manual::Builder::new().compile(&[&greeter_service]);
//!     tonic_build::manual::Builder::new().compile(&[greeter_service]);
//!     Ok(())
//! }
//! ```
// This module forked from https://github.com/hyperium/tonic/commit/1c5150aaf62d6e72ce6c07966a9f19ceedb52702

use crate::code_gen::CodeGenBuilder;
use proc_macro2::TokenStream;
use quote::quote;
use std::{
    fs,
    path::{Path, PathBuf},
};
use tonic_build::{manual, Service};

struct ServiceGenerator {
    // builder: Builder,
    definitions: TokenStream,
}

impl ServiceGenerator {
    fn generate(&mut self, service: &manual::Service, rewrite_crate: &str) {
        let definition = CodeGenBuilder::new()
            .emit_package(true)
            .compile_well_known_types(false)
            .generate_server_definition(service, rewrite_crate, "");

        self.definitions.extend(definition);
    }

    fn finalize(&mut self, buf: &mut String) {
        if !self.definitions.is_empty() {
            let definitions = &self.definitions;

            let server_definitions = quote::quote! {
                #definitions
            };

            let ast: syn::File =
                syn::parse2(server_definitions).expect("not a valid tokenstream");
            let code = prettyplease::unparse(&ast);
            buf.push_str(&code);

            self.definitions = TokenStream::default();
        }
    }
}

/// Service generator builder.
#[derive(Debug, Default)]
pub struct Builder {
    rewrite_crate_name: Option<String>,
    out_dir: Option<PathBuf>,
    emit_composite_package: bool,
}

impl Builder {
    /// Create a new Builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Rewrite `crate::` references to provided crate
    ///
    pub fn rewrite_crate(mut self, crate_name: &str) -> Self {
        self.rewrite_crate_name = Some(crate_name.to_string());
        self
    }

    /// Set the output directory to generate code to.
    ///
    /// Defaults to the `OUT_DIR` environment variable.
    pub fn out_dir(mut self, out_dir: impl AsRef<Path>) -> Self {
        self.out_dir = Some(out_dir.as_ref().to_path_buf());
        self
    }

    /// Set whether to emit a composite package.
    ///
    /// Defaults to false
    pub fn emit_composite_package(mut self, emit_composite_package: bool) -> Self {
        self.emit_composite_package = emit_composite_package;
        self
    }

    /// Performs code generation for the provided services.
    ///
    /// Generated services will be output into the directory specified by `out_dir`
    /// with files named `<package_name>.<service_name>.sdk.rs`.
    pub fn compile(self, services: &[&manual::Service]) {
        let out_dir = if let Some(out_dir) = self.out_dir.as_ref() {
            fs::create_dir_all(out_dir).unwrap_or_else(|_| {
                panic!("failed to create out dir: {}", out_dir.display())
            });
            out_dir.clone()
        } else {
            PathBuf::from(std::env::var("OUT_DIR").unwrap())
        };

        // If provided, rewrites `crate::mod::Type` references to `{name}::mod::Type`
        let rewrite_crate_name = if let Some(name) = self.rewrite_crate_name.as_ref() {
            name
        } else {
            "crate"
        };

        let mut generator = ServiceGenerator {
            // builder: self,
            definitions: TokenStream::default(),
        };

        for service in services {
            let mut output = String::new();
            generator.generate(service, rewrite_crate_name);
            generator.finalize(&mut output);

            let out_file = out_dir.join(out_file(service));
            fs::write(&out_file, output)
                .unwrap_or_else(|_| panic!("failed to write: {}", out_file.display()));
        }

        if self.emit_composite_package {
            let out_file = out_dir.join("packages.sdk.rs");
            let output = generate_composite_package(services);
            let ast = syn::parse2(output).unwrap();
            let code = prettyplease::unparse(&ast);

            fs::write(&out_file, code)
                .unwrap_or_else(|_| panic!("failed to write: {}", out_file.display()));
        }
    }
}

fn out_file(service: &manual::Service) -> String {
    format!("{}.{}.sdk.rs", service.package(), service.name())
}

fn generate_composite_package(services: &[&manual::Service]) -> TokenStream {
    let mut includes = TokenStream::new();
    let mut rpc_calls = TokenStream::new();

    for service in services {
        let service_name = syn::Lit::Str(syn::LitStr::new(
            &out_file(service),
            proc_macro2::Span::call_site(),
        ));
        includes.extend(quote! {
            include!(#service_name);
        });

        let call = crate::server::server_fn_ident(service.name());
        rpc_calls.extend(quote! { #call(), });
    }

    let rpcs = quote! {
        vec![#rpc_calls]
    };

    quote! {
        #includes

        pub fn definitions() -> Vec<schema_builder::code_gen_types::SdkGeneratorStruct> {
            #rpcs
        }
    }
}
