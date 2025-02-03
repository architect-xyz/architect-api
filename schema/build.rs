use schema_builder::code_gen_types::*;
use std::{io::Write, path::PathBuf};

mod defs {
    include!("../src/grpc/generated/packages.sdk.rs");
}

fn main() {
    let config = Config { pretty: true };
    let mut out_file = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    out_file.push("../schema.json");

    let mut f = std::fs::File::create(out_file).unwrap();
    f.write_all("[\n".as_bytes()).unwrap();

    let defs = defs::definitions();
    let total = defs.len();
    defs.into_iter().map(|sdk_struct| emit(&sdk_struct, &config)).enumerate().for_each(
        |(index, json)| {
            let postfix = if (index + 1) < total { ",\n\n" } else { "\n" };
            f.write_all(format!("{json}{postfix}").as_bytes()).unwrap()
        },
    );

    f.write_all("]".as_bytes()).unwrap();
}
