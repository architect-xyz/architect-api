#[cfg(feature = "tonic")]
fn build_grpc_stubs() {
    let json_codec = "crate::utils::grpc::json_codec::JsonCodec";
    let json_symbology_service = tonic_build::manual::Service::builder()
        .name("Symbology")
        .package("json.architect")
        .method(
            tonic_build::manual::Method::builder()
                .name("symbology_snapshot")
                .route_name("SymbologySnapshot")
                .input_type("crate::external::symbology::SymbologySnapshotRequest")
                .output_type("crate::external::symbology::SymbologySnapshot")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("subscribe_symbology_updates")
                .route_name("SubscribeSymbologyUpdates")
                .input_type(
                    "crate::external::symbology::SubscribeSymbologyUpdatesRequest",
                )
                .output_type("crate::external::symbology::SymbologyUpdate")
                .server_streaming()
                .codec_path(json_codec)
                .build(),
        )
        .build();
    let json_marketdata_service = tonic_build::manual::Service::builder()
        .name("Marketdata")
        .package("json.architect")
        .method(
            tonic_build::manual::Method::builder()
                .name("l1_book_snapshot")
                .route_name("L1BookSnapshot")
                .input_type("crate::external::marketdata::L1BookSnapshotRequest")
                .output_type("crate::external::marketdata::L1BookSnapshot")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("l1_book_snapshots")
                .route_name("L1BookSnapshots")
                .input_type("crate::external::marketdata::L1BookSnapshotsRequest")
                .output_type("crate::external::marketdata::L1BookSnapshots")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("subscribe_l1_book_snapshots")
                .route_name("SubscribeL1BookSnapshots")
                .input_type(
                    "crate::external::marketdata::SubscribeL1BookSnapshotsRequest",
                )
                .output_type("crate::external::marketdata::L1BookSnapshot")
                .server_streaming()
                .codec_path(json_codec)
                .build(),
        )
        .build();
    tonic_build::manual::Builder::new()
        .compile(&[json_symbology_service, json_marketdata_service]);
}

fn main() {
    #[cfg(feature = "tonic")]
    build_grpc_stubs();
}
