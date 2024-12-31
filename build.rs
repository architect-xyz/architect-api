#[cfg(feature = "tonic")]
fn build_grpc_stubs() {
    let json_codec = "crate::grpc::json_codec::JsonCodec";
    let json_health_service = tonic_build::manual::Service::builder()
        .name("Health")
        .package("json.architect")
        .method(
            tonic_build::manual::Method::builder()
                .name("check")
                .route_name("Check")
                .input_type("crate::grpc::health::HealthCheckRequest")
                .output_type("crate::grpc::health::HealthCheckResponse")
                .codec_path(json_codec)
                .build(),
        )
        .build();
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
    let json_symbology_v2_service = tonic_build::manual::Service::builder()
        .name("SymbologyV2")
        .package("json.architect")
        .method(
            tonic_build::manual::Method::builder()
                .name("subscribe_symbology_v2")
                .route_name("SubscribeSymbologyV2")
                .input_type("crate::external::symbology_v2::SubscribeSymbologyV2")
                .output_type("crate::external::symbology_v2::SymbologyTransaction")
                .server_streaming()
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("upload_symbology_v2")
                .route_name("UploadSymbologyV2")
                .input_type("crate::external::symbology_v2::SymbologyTransaction")
                .output_type("crate::external::symbology_v2::UploadSymbologyV2Response")
                .client_streaming()
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("prune_expired_symbols")
                .route_name("PruneExpiredSymbols")
                .input_type("crate::external::symbology_v2::PruneExpiredSymbolsRequest")
                .output_type("crate::external::symbology_v2::PruneExpiredSymbolsResponse")
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
        .method(
            tonic_build::manual::Method::builder()
                .name("subscribe_l2_book_updates")
                .route_name("SubscribeL2BookUpdates")
                .input_type("crate::external::marketdata::SubscribeL2BookUpdatesRequest")
                .output_type("crate::external::marketdata::L2BookUpdate")
                .server_streaming()
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("l2_book_snapshot")
                .route_name("L2BookSnapshot")
                .input_type("crate::external::marketdata::L2BookSnapshotRequest")
                .output_type("crate::external::marketdata::L2BookSnapshot")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("subscribe_candles")
                .route_name("SubscribeCandles")
                .input_type("crate::external::marketdata::SubscribeCandlesRequest")
                .output_type("crate::external::marketdata::Candle")
                .server_streaming()
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("subscribe_many_candles")
                .route_name("SubscribeManyCandles")
                .input_type("crate::external::marketdata::SubscribeManyCandlesRequest")
                .output_type("crate::external::marketdata::Candle")
                .server_streaming()
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("subscribe_trades")
                .route_name("SubscribeTrades")
                .input_type("crate::external::marketdata::SubscribeTradesRequest")
                .output_type("crate::external::marketdata::Trade")
                .server_streaming()
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("market_status")
                .route_name("MarketStatus")
                .input_type("crate::external::marketdata::MarketStatusRequest")
                .output_type("crate::external::marketdata::MarketStatus")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("ticker")
                .route_name("Ticker")
                .input_type("crate::external::marketdata::TickerRequest")
                .output_type("crate::external::marketdata::Ticker")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("subscribe_tickers")
                .route_name("SubscribeTickers")
                .input_type("crate::external::marketdata::SubscribeTickersRequest")
                .output_type("crate::external::marketdata::Ticker")
                .server_streaming()
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("subscribe_liquidations")
                .route_name("SubscribeLiquidations")
                .input_type("crate::external::marketdata::SubscribeLiquidationsRequest")
                .output_type("crate::external::marketdata::Liquidation")
                .codec_path(json_codec)
                .server_streaming()
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("exchange_specific_fields")
                .route_name("ExchangeSpecificFields")
                .input_type("crate::external::marketdata::ExchangeSpecificFieldsRequest")
                .output_type("crate::external::marketdata::ExchangeSpecificFields")
                .codec_path(json_codec)
                .build(),
        )
        .build();
    tonic_build::manual::Builder::new().compile(&[
        json_health_service,
        json_symbology_service,
        json_symbology_v2_service,
        json_marketdata_service,
    ]);
}

fn main() {
    #[cfg(feature = "tonic")]
    build_grpc_stubs();
}
