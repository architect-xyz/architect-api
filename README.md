# architect-api

## GraphQL support via Juniper

Trivial types, and types with only one obvious schema, come with implementations of 
GraphQL traits for use with the `juniper` crate.  These can be enabled using the
`juniper` feature flag.  In general, however, the user should write their own wrapper
types for a bespoke GraphQL schema.