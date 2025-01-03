use schemars::schema::RootSchema;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Deserialize)]
pub enum RpcType {
    ClientStreaming,
    ServerStreaming,
    Unary,
}

#[derive(Serialize, Deserialize)]
pub struct RpcDefinition {
    #[serde(rename = "type")]
    pub rpc_type: RpcType,
    pub route: String,
    pub request_type: RootSchema,
    pub response_type: RootSchema,
}

#[derive(Serialize, Deserialize)]
pub struct SdkGeneratorStruct {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub name: String,
    pub rpcs: Vec<RpcDefinition>,
}

pub struct Config {
    // Pretty print json
    pub pretty: bool,
}

pub fn emit(definition: &SdkGeneratorStruct, config: &Config) -> String {
    if config.pretty {
        serde_json::to_string_pretty(definition).expect("derp")
    } else {
        serde_json::to_string(definition).expect("derp")
    }
}

impl Serialize for RpcType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let output = match *self {
            RpcType::ClientStreaming => "client_stream",
            RpcType::ServerStreaming => "stream",
            RpcType::Unary => "unary",
        };
        serializer.serialize_str(output)
    }
}
