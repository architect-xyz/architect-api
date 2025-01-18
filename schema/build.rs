use schema_builder::code_gen_types::*;
use std::{io::Write, path::PathBuf};

mod codegen_definitions {
    use schema_builder::code_gen_types::SdkGeneratorStruct;

    include!("./generated/json.architect.Health.sdk.rs");
    include!("./generated/json.architect.Symbology.sdk.rs");
    include!("./generated/json.architect.SymbologyV2.sdk.rs");
    include!("./generated/json.architect.Marketdata.sdk.rs");
    include!("./generated/json.architect.MarketdataSnapshots.sdk.rs");
    include!("./generated/json.architect.Orderflow.sdk.rs");

    pub fn definitions() -> Vec<SdkGeneratorStruct> {
        vec![
            get_health_server_definition(),
            get_symbology_server_definition(),
            get_symbology_v2_server_definition(),
            get_marketdata_server_definition(),
            get_marketdata_snapshots_server_definition(),
            get_orderflow_server_definition(),
        ]
    }
}

fn main() {
    let config = Config { pretty: true };

    let mut out_file = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    out_file.push("../schema.json");

    let mut f = std::fs::File::create(out_file).unwrap();
    f.write_all("[\n".as_bytes()).unwrap();

    let defs = codegen_definitions::definitions();
    let total = defs.len();
    defs.into_iter().map(|sdk_struct| emit(&sdk_struct, &config)).enumerate().for_each(
        |(index, json)| {
            let postfix = if (index + 1) < total { ",\n\n" } else { "\n" };
            f.write_all(format!("{json}{postfix}").as_bytes()).unwrap()
        },
    );

    f.write_all("]".as_bytes()).unwrap();
}
