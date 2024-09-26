fn main() {
    let json_codec = "crate::utils::json_codec::JsonCodec";
    let json_marketdata_service = tonic_build::manual::Service::builder()
        .name("Marketdata")
        .package("json.marketdata")
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
    tonic_build::manual::Builder::new().compile(&[json_marketdata_service]);
}
