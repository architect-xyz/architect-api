# architect-api

## GraphQL support via Juniper

Trivial types, and types with only one obvious schema, come with implementations of 
GraphQL traits for use with the `juniper` crate.  These can be enabled using the
`juniper` feature flag.  In general, however, the user should write their own wrapper
types for a bespoke GraphQL schema.

## JSONSchema generation

We generate a metadata + JSONSchema representation of our gRPC APIs to assist in generating client SDKs.

This is accomplished via two modules:

* `schema/builder` -- logic and types for generating schema definitions from `tonic_build` service definitions
* `schema` -- a stub crate/phony target that uses the generated schema definitions to build a schema.json file

> [!NOTE]  
> New gRPC stubs/services must be added to `schema/build.rs` as well as `build.rs` to be included in the generated `schema.json` file.

The dependency graph induces a build order: `schema/builder` -> `architect-api` -> `schema`.  This build order naturally regenerates the schema.json file whenever this crate changes.

Downstream SDKs:

* [architect-ts](https://github.com/architect-xyz/architect-ts) 
* [architect-py](https://github.com/architect-xyz/architect-py) 

