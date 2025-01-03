//! [2024-12-30] dkasten: copy of tonic-build/src/codegen.rs at
//! https://github.com/hyperium/tonic/commit/aff1daf65d9a0d55b92719318eba2b5a4769c4e1

use proc_macro2::TokenStream;
use std::collections::HashSet;
use tonic_build::{Attributes, Service};

/// Builder for the generic code generation of server and clients.
#[derive(Debug)]
pub struct CodeGenBuilder {
    emit_package: bool,
    compile_well_known_types: bool,
    attributes: Attributes,
    build_transport: bool,
    disable_comments: HashSet<String>,
    use_arc_self: bool,
    generate_default_stubs: bool,
}

impl CodeGenBuilder {
    /// Create a new code gen builder with default options.
    pub fn new() -> Self {
        Default::default()
    }

    /// Enable code generation to emit the package name.
    pub fn emit_package(&mut self, enable: bool) -> &mut Self {
        self.emit_package = enable;
        self
    }

    /// Attributes that will be added to `mod` and `struct` items.
    ///
    /// Reference [`Attributes`] for more information.
    pub fn attributes(&mut self, attributes: Attributes) -> &mut Self {
        self.attributes = attributes;
        self
    }

    /// Enable transport code to be generated, this requires `tonic`'s `transport`
    /// feature.
    ///
    /// This allows codegen level control of generating the transport code and
    /// is a work around when other crates in a workspace enable this feature.
    pub fn build_transport(&mut self, build_transport: bool) -> &mut Self {
        self.build_transport = build_transport;
        self
    }

    /// Enable compiling well knonw types, this will force codegen to not
    /// use the well known types from `prost-types`.
    pub fn compile_well_known_types(&mut self, enable: bool) -> &mut Self {
        self.compile_well_known_types = enable;
        self
    }

    /// Disable comments based on a proto path.
    pub fn disable_comments(&mut self, disable_comments: HashSet<String>) -> &mut Self {
        self.disable_comments = disable_comments;
        self
    }

    /// Emit `Arc<Self>` instead of `&self` in servica trait.
    pub fn use_arc_self(&mut self, enable: bool) -> &mut Self {
        self.use_arc_self = enable;
        self
    }

    /// Enable or disable returning automatic unimplemented gRPC error code for generated traits.
    pub fn generate_default_stubs(&mut self, generate_default_stubs: bool) -> &mut Self {
        self.generate_default_stubs = generate_default_stubs;
        self
    }

    /// Generate server interface definition based on `Service`.
    ///
    /// This takes some `Service` and will generate a `TokenStream` that contains
    /// a public module with the generated service definition.
    pub fn generate_server_definition(
        &self,
        service: &impl Service,
        rewrite_crate: &str,
        proto_path: &str,
    ) -> TokenStream {
        crate::server::generate_definition(
            service,
            self.emit_package,
            rewrite_crate,
            proto_path,
            self.compile_well_known_types,
            &self.attributes,
            &self.disable_comments,
            self.use_arc_self,
            self.generate_default_stubs,
        )
    }
}

impl Default for CodeGenBuilder {
    fn default() -> Self {
        Self {
            emit_package: true,
            compile_well_known_types: false,
            attributes: Attributes::default(),
            build_transport: true,
            disable_comments: HashSet::default(),
            use_arc_self: false,
            generate_default_stubs: false,
        }
    }
}
