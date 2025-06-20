#[macro_export]
macro_rules! json_schema_is_string {
    ($type:ident) => {
        impl schemars::JsonSchema for $type {
            fn schema_name() -> String {
                stringify!($type).to_owned()
            }

            fn json_schema(
                _gen: &mut schemars::r#gen::SchemaGenerator,
            ) -> schemars::schema::Schema {
                schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::String.into()),
                    ..Default::default()
                }
                .into()
            }

            fn is_referenceable() -> bool {
                true
            }
        }
    };
    ($type:ident, $format:literal) => {
        impl schemars::JsonSchema for $type {
            fn schema_name() -> String {
                stringify!($type).to_owned()
            }

            fn json_schema(
                _gen: &mut schemars::r#gen::SchemaGenerator,
            ) -> schemars::schema::Schema {
                schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::String.into()),
                    format: Some($format.to_owned()),
                    ..Default::default()
                }
                .into()
            }

            fn is_referenceable() -> bool {
                true
            }
        }
    };
}
