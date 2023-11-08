// CR estokes: move to netidx
/// Implement Into<Value> and FromValue using Pack
#[macro_export]
macro_rules! packed_value {
    ($name:ident) => {
        impl Into<netidx::protocol::value::Value> for $name {
            fn into(self) -> netidx::protocol::value::Value {
                // this will never fail
                netidx::protocol::value::Value::Bytes(
                    netidx::utils::pack(&self).unwrap().freeze(),
                )
            }
        }

        impl netidx::protocol::value::FromValue for $name {
            fn from_value(v: netidx::protocol::value::Value) -> anyhow::Result<Self> {
                match v {
                    netidx::protocol::value::Value::Bytes(mut b) => {
                        Ok(netidx::pack::Pack::decode(&mut b)?)
                    }
                    _ => anyhow::bail!("invalid value, expected a bytes {:?}", v),
                }
            }
        }
    };
}
