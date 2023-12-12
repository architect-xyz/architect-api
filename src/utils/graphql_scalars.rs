//! Custom scalars for GraphQL

use derive::Newtype;

#[derive(Debug, Clone, Copy, Newtype, juniper::GraphQLScalar)]
#[newtype(Deref, DerefMut)]
pub struct U8(pub u8);

impl From<u8> for U8 {
    fn from(v: u8) -> Self {
        Self(v)
    }
}

impl U8 {
    fn to_output<S: juniper::ScalarValue>(&self) -> juniper::Value<S> {
        juniper::Value::scalar(self.0 as i32)
    }

    fn from_input<S>(v: &juniper::InputValue<S>) -> Result<Self, String>
    where
        S: juniper::ScalarValue,
    {
        v.as_int_value()
            .map(|i| u8::try_from(i))
            .ok_or_else(|| format!("Expected `Int`, found: {v}"))?
            .map(Self)
            .map_err(|e| e.to_string())
    }

    fn parse_token<S>(value: juniper::ScalarToken<'_>) -> juniper::ParseScalarResult<S>
    where
        S: juniper::ScalarValue,
    {
        <i32 as juniper::ParseScalarValue<S>>::from_str(value)
    }
}
