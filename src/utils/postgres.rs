/// Macro that implements the `ToSql` trait for a type using its string representation.
#[macro_export]
macro_rules! to_sql_str {
    ($ty:ident) => {
        impl postgres_types::ToSql for $ty {
            postgres_types::to_sql_checked!();

            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut bytes::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                Into::<&str>::into(*self).to_sql(ty, out)
            }

            fn accepts(ty: &postgres_types::Type) -> bool {
                String::accepts(ty)
            }
        }
    };
}
