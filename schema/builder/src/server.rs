//! [2024-12-30] dkasten: fork of tonic-build/src/server.rs at
//! https://github.com/hyperium/tonic/commit/60b131d2cd066dc0ee386175bf7c7124c23c72f2
//!
//! The only relevant updates here compared to upstream is ensuring the types
//! continue to match.  Upstream, this file generate rust code for rust usage.
//!
//! We are emitting a JSONSchema representation for other codegen tools to use,
//! e.g. generating Python and TypeScript SDKs.

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use std::collections::HashSet;
use syn::{Ident, Lit, LitStr};
use tonic_build::{Attributes, Method, Service};

#[allow(clippy::too_many_arguments)]
pub(crate) fn generate_definition<T: Service>(
    service: &T,
    emit_package: bool,
    rewrite_crate_name: &str,
    proto_path: &str,
    compile_well_known_types: bool,
    _attributes: &Attributes,
    _disable_comments: &HashSet<String>,
    use_arc_self: bool,
    generate_default_stubs: bool,
) -> TokenStream {
    let (schemas, definitions) = generate_methods(
        service,
        rewrite_crate_name,
        emit_package,
        proto_path,
        compile_well_known_types,
        use_arc_self,
        generate_default_stubs,
    );

    let server_fn = quote::format_ident!(
        "get_{}_server_definition",
        naive_snake_case(service.name())
    );
    let server_service_str = service.name();
    let rpcs = quote! {
        vec![#definitions]
    };

    quote! {
        pub fn #server_fn() -> schema_builder::code_gen_types::SdkGeneratorStruct {
            #schemas

            schema_builder::code_gen_types::SdkGeneratorStruct {
                schema: "jsonschema".to_string(),
                name: #server_service_str.to_string(),
                rpcs: #rpcs,
            }
        }
    }
}

fn generate_methods<T: Service>(
    service: &T,
    rewrite_crate_name: &str,
    emit_package: bool,
    proto_path: &str,
    compile_well_known_types: bool,
    use_arc_self: bool,
    generate_default_stubs: bool,
) -> (TokenStream, TokenStream) {
    let mut schema_stream = TokenStream::new();
    let mut definition_stream = TokenStream::new();

    for method in service.methods() {
        let path = format_method_path(service, method, emit_package);
        //  method_path = Lit::Str("/json.helloworld.Greeter/SayHello", Span)
        let method_path = Lit::Str(LitStr::new(&path, Span::call_site()));
        let ident = quote::format_ident!("{}", method.name());
        let server_trait = quote::format_ident!("{}", service.name());

        let (schemas, definitions) =
            match (method.client_streaming(), method.server_streaming()) {
                (false, false) => generate_unary(
                    method,
                    rewrite_crate_name,
                    proto_path,
                    method_path,
                    compile_well_known_types,
                    ident,
                    server_trait,
                    use_arc_self,
                ),

                (false, true) => generate_server_streaming(
                    method,
                    rewrite_crate_name,
                    proto_path,
                    method_path,
                    compile_well_known_types,
                    ident.clone(),
                    server_trait,
                    use_arc_self,
                    generate_default_stubs,
                ),
                (true, false) => generate_client_streaming(
                    method,
                    rewrite_crate_name,
                    proto_path,
                    method_path,
                    compile_well_known_types,
                    ident.clone(),
                    server_trait,
                    use_arc_self,
                ),

                (true, true) => generate_streaming(
                    method,
                    rewrite_crate_name,
                    proto_path,
                    method_path,
                    compile_well_known_types,
                    ident.clone(),
                    server_trait,
                    use_arc_self,
                    generate_default_stubs,
                ),
            };

        schema_stream.extend(schemas);
        definition_stream.extend(definitions);
    }

    (schema_stream, definition_stream)
}

fn generate_unary<T: Method>(
    method: &T,
    crate_name: &str,
    proto_path: &str,
    method_path: Lit,
    compile_well_known_types: bool,
    // The name from `tonic_build::manual::Method::builder().name("say_hello")`. Ident(say_hello);
    method_ident: Ident,
    // The name from `tonic_build::manual::Service::builder().name("Greeter")`. Ident(Greeter);
    _server_trait: Ident,
    _use_arc_self: bool,
) -> (TokenStream, TokenStream) {
    let (request, response) =
        request_response_name(method, crate_name, proto_path, compile_well_known_types);
    let request_ident = quote::format_ident!("{}Request", method_ident);
    let response_ident = quote::format_ident!("{}Response", method_ident);

    let schemas = quote! {
        #[allow(non_snake_case)]
        let #request_ident = schemars::schema_for!(#request);
        #[allow(non_snake_case)]
        let #response_ident = schemars::schema_for!(#response);
    };
    let definitions = quote! {
        schema_builder::code_gen_types::RpcDefinition {
            rpc_type: schema_builder::code_gen_types::RpcType::Unary,
            route: #method_path.to_string(),
            request_type: #request_ident,
            response_type: #response_ident,
        },
    };

    (schemas, definitions)
}

#[allow(clippy::too_many_arguments)]
fn generate_server_streaming<T: Method>(
    method: &T,
    crate_name: &str,
    proto_path: &str,
    method_path: Lit,
    compile_well_known_types: bool,
    method_ident: Ident,
    _server_trait: Ident,
    _use_arc_self: bool,
    _generate_default_stubs: bool,
) -> (TokenStream, TokenStream) {
    let (request, response) =
        request_response_name(method, crate_name, proto_path, compile_well_known_types);
    let request_ident = quote::format_ident!("{}Request", method_ident);
    let response_ident = quote::format_ident!("{}Response", method_ident);
    // println!("cargo:warning=hello codec {:?}", method.codec_path());

    let schemas = quote! {
        #[allow(non_snake_case)]
        let #request_ident = schemars::schema_for!(#request);
        #[allow(non_snake_case)]
        let #response_ident = schemars::schema_for!(#response);
    };
    let definitions = quote! {
        schema_builder::code_gen_types::RpcDefinition {
            rpc_type: schema_builder::code_gen_types::RpcType::ServerStreaming,
            route: #method_path.to_string(),
            request_type: #request_ident,
            response_type: #response_ident,
        },
    };

    (schemas, definitions)
}

#[allow(clippy::too_many_arguments)]
fn generate_client_streaming<T: Method>(
    method: &T,
    crate_name: &str,
    proto_path: &str,
    method_path: Lit,
    compile_well_known_types: bool,
    method_ident: Ident,
    _server_trait: Ident,
    _use_arc_self: bool,
) -> (TokenStream, TokenStream) {
    let (request, response) =
        request_response_name(method, crate_name, proto_path, compile_well_known_types);
    let request_ident = quote::format_ident!("{}Request", method_ident);
    let response_ident = quote::format_ident!("{}Response", method_ident);
    // println!("cargo:warning=hello codec {:?}", method.codec_path());

    let schemas = quote! {
        #[allow(non_snake_case)]
        let #request_ident = schemars::schema_for!(#request);
        #[allow(non_snake_case)]
        let #response_ident = schemars::schema_for!(#response);
    };
    let definitions = quote! {
        schema_builder::code_gen_types::RpcDefinition {
            rpc_type: schema_builder::code_gen_types::RpcType::ClientStreaming,
            route: #method_path.to_string(),
            request_type: #request_ident,
            response_type: #response_ident,
        },
    };

    (schemas, definitions)
}

#[allow(clippy::too_many_arguments)]
fn generate_streaming<T: Method>(
    method: &T,
    crate_name: &str,
    proto_path: &str,
    method_path: Lit,
    compile_well_known_types: bool,
    method_ident: Ident,
    _server_trait: Ident,
    _use_arc_self: bool,
    _generate_default_stubs: bool,
) -> (TokenStream, TokenStream) {
    let (request, response) =
        request_response_name(method, crate_name, proto_path, compile_well_known_types);
    let request_ident = quote::format_ident!("{}Request", method_ident);
    let response_ident = quote::format_ident!("{}Response", method_ident);
    // println!("cargo:warning=hello codec {:?}", method.codec_path());

    let schemas = quote! {
        #[allow(non_snake_case)]
        let #request_ident = schemars::schema_for!(#request);
        #[allow(non_snake_case)]
        let #response_ident = schemars::schema_for!(#response);
    };
    let definitions = quote! {
        schema_builder::code_gen_types::RpcDefinition {
            rpc_type: schema_builder::code_gen_types::RpcType::BidirectionalStreaming,
            route: #method_path.to_string(),
            request_type: #request_ident,
            response_type: #response_ident,
        },
    };

    (schemas, definitions)
}

fn naive_snake_case(name: &str) -> String {
    let mut s = String::new();
    let mut it = name.chars().peekable();

    while let Some(x) = it.next() {
        s.push(x.to_ascii_lowercase());
        if let Some(y) = it.peek() {
            if y.is_uppercase() {
                s.push('_');
            }
        }
    }

    s
}

// wrapper around https://github.com/hyperium/tonic/blob/a8c1e39600772b240d54b8adfaed03e7879643aa/tonic-build/src/manual.rs#L220-L232 to support `crate::` rewriting on input_type and and output_type
fn rewrite_crate_name(stream: TokenStream, crate_name: &str) -> TokenStream {
    let str = stream.to_string();
    syn::parse_str::<syn::Path>(&str.replace("crate", crate_name))
        .unwrap()
        .to_token_stream()
}

fn request_response_name<T: Method>(
    method: &T,
    crate_name: &str,
    proto_path: &str,
    compile_well_known_types: bool,
) -> (TokenStream, TokenStream) {
    let (req, res) = method.request_response_name(proto_path, compile_well_known_types);
    let request = rewrite_crate_name(req, crate_name);
    let response = rewrite_crate_name(res, crate_name);
    (request, response)
}

fn format_method_path<T: Service>(
    service: &T,
    method: &T::Method,
    emit_package: bool,
) -> String {
    format!("/{}/{}", format_service_name(service, emit_package), method.identifier())
}

fn format_service_name<T: Service>(service: &T, emit_package: bool) -> String {
    let package = if emit_package { service.package() } else { "" };
    format!(
        "{}{}{}",
        package,
        if package.is_empty() { "" } else { "." },
        service.identifier(),
    )
}
