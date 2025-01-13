use schema_builder::code_gen_types::*;
use std::{io::Write, path::PathBuf};

mod codegen_definitions {
    include!("./generated/json.architect.Health.sdk.rs");
    include!("./generated/json.architect.Symbology.sdk.rs");
    include!("./generated/json.architect.SymbologyV2.sdk.rs");
    include!("./generated/json.architect.Marketdata.sdk.rs");
    include!("./generated/json.architect.Orderflow.sdk.rs");
}

fn main() {
    let config = Config { pretty: true };
    let health = emit(&codegen_definitions::get_health_server_definition(), &config);
    let symbology =
        emit(&codegen_definitions::get_symbology_server_definition(), &config);
    let symbology_v2 =
        emit(&codegen_definitions::get_symbology_v2_server_definition(), &config);
    let marketdata =
        emit(&codegen_definitions::get_marketdata_server_definition(), &config);
    let orderflow =
        emit(&codegen_definitions::get_orderflow_server_definition(), &config);
    let mut out_file = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    out_file.push("../schema.json");
    let mut f = std::fs::File::create(out_file).unwrap();
    f.write_all(format!("[{health},\n\n").as_bytes()).unwrap();
    f.write_all(format!("{symbology},\n\n").as_bytes()).unwrap();
    f.write_all(format!("{symbology_v2},\n\n").as_bytes()).unwrap();
    f.write_all(format!("{marketdata},\n\n").as_bytes()).unwrap();
    f.write_all(format!("{orderflow}]").as_bytes()).unwrap();
}
