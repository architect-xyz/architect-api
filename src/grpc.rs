// CR alee: move utils/grpc codecs up here

pub mod json_service {
    include!(concat!(env!("OUT_DIR"), "/json.architect.Symbology.rs"));
    include!(concat!(env!("OUT_DIR"), "/json.architect.SymbologyV2.rs"));
    include!(concat!(env!("OUT_DIR"), "/json.architect.Marketdata.rs"));
}
