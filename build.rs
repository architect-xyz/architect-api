#[cfg(feature = "tonic")]
fn build_grpc_stubs() {
    use std::path::PathBuf;
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
                .name("symbols")
                .route_name("Symbols")
                .input_type("crate::symbology::protocol::SymbolsRequest")
                .output_type("crate::symbology::protocol::SymbolsResponse")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("symbology")
                .route_name("Symbology")
                .input_type("crate::symbology::protocol::SymbologyRequest")
                .output_type("crate::symbology::protocol::SymbologySnapshot")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("subscribe_symbology")
                .route_name("SubscribeSymbology")
                .input_type("crate::symbology::protocol::SubscribeSymbology")
                .output_type("crate::symbology::protocol::SymbologyUpdate")
                .server_streaming()
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("upload_symbology")
                .route_name("UploadSymbology")
                .input_type("crate::symbology::protocol::UploadSymbologyRequest")
                .output_type("crate::symbology::protocol::UploadSymbologyResponse")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("prune_expired_symbols")
                .route_name("PruneExpiredSymbols")
                .input_type("crate::symbology::protocol::PruneExpiredSymbolsRequest")
                .output_type("crate::symbology::protocol::PruneExpiredSymbolsResponse")
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
                .input_type("crate::marketdata::L1BookSnapshotRequest")
                .output_type("crate::marketdata::L1BookSnapshot")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("l1_book_snapshots")
                .route_name("L1BookSnapshots")
                .input_type("crate::marketdata::L1BookSnapshotsRequest")
                .output_type("crate::marketdata::L1BookSnapshots")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("subscribe_l1_book_snapshots")
                .route_name("SubscribeL1BookSnapshots")
                .input_type("crate::marketdata::SubscribeL1BookSnapshotsRequest")
                .output_type("crate::marketdata::L1BookSnapshot")
                .server_streaming()
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("subscribe_l2_book_updates")
                .route_name("SubscribeL2BookUpdates")
                .input_type("crate::marketdata::SubscribeL2BookUpdatesRequest")
                .output_type("crate::marketdata::L2BookUpdate")
                .server_streaming()
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("l2_book_snapshot")
                .route_name("L2BookSnapshot")
                .input_type("crate::marketdata::L2BookSnapshotRequest")
                .output_type("crate::marketdata::L2BookSnapshot")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("subscribe_candles")
                .route_name("SubscribeCandles")
                .input_type("crate::marketdata::SubscribeCandlesRequest")
                .output_type("crate::marketdata::Candle")
                .server_streaming()
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("subscribe_many_candles")
                .route_name("SubscribeManyCandles")
                .input_type("crate::marketdata::SubscribeManyCandlesRequest")
                .output_type("crate::marketdata::Candle")
                .server_streaming()
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("subscribe_current_candles")
                .route_name("SubscribeCurrentCandles")
                .input_type("crate::marketdata::SubscribeCurrentCandlesRequest")
                .output_type("crate::marketdata::Candle")
                .server_streaming()
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("subscribe_trades")
                .route_name("SubscribeTrades")
                .input_type("crate::marketdata::SubscribeTradesRequest")
                .output_type("crate::marketdata::Trade")
                .server_streaming()
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("market_status")
                .route_name("MarketStatus")
                .input_type("crate::marketdata::MarketStatusRequest")
                .output_type("crate::marketdata::MarketStatus")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("ticker")
                .route_name("Ticker")
                .input_type("crate::marketdata::TickerRequest")
                .output_type("crate::marketdata::Ticker")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("tickers")
                .route_name("Tickers")
                .input_type("crate::marketdata::TickersRequest")
                .output_type("crate::marketdata::TickersResponse")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("subscribe_tickers")
                .route_name("SubscribeTickers")
                .input_type("crate::marketdata::SubscribeTickersRequest")
                .output_type("crate::marketdata::TickerUpdate")
                .server_streaming()
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("subscribe_liquidations")
                .route_name("SubscribeLiquidations")
                .input_type("crate::marketdata::SubscribeLiquidationsRequest")
                .output_type("crate::marketdata::Liquidation")
                .codec_path(json_codec)
                .server_streaming()
                .build(),
        )
        .build();
    let json_marketdata_snapshots_service = tonic_build::manual::Service::builder()
        .name("MarketdataSnapshots")
        .package("json.architect")
        .method(
            tonic_build::manual::Method::builder()
                .name("marketdata_snapshot")
                .route_name("MarketdataSnapshot")
                .input_type("crate::marketdata::snapshots::MarketdataSnapshotRequest")
                .output_type("crate::marketdata::snapshots::MarketdataSnapshot")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("marketdata_snapshots")
                .route_name("MarketdataSnapshots")
                .input_type("crate::marketdata::snapshots::MarketdataSnapshotsRequest")
                .output_type("crate::marketdata::snapshots::MarketdataSnapshotsResponse")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("subscribe_marketdata_snapshots")
                .route_name("SubscribeMarketdataSnapshots")
                .input_type(
                    "crate::marketdata::snapshots::SubscribeMarketdataSnapshotsRequest",
                )
                .output_type(
                    "crate::marketdata::snapshots::SubscribeMarketdataSnapshotsResponse",
                )
                .server_streaming()
                .codec_path(json_codec)
                .build(),
        )
        .build();
    let json_orderflow_service = tonic_build::manual::Service::builder()
        .name("Orderflow")
        .package("json.architect")
        .method(
            tonic_build::manual::Method::builder()
                .name("orderflow")
                .route_name("Orderflow")
                .input_type("crate::orderflow::OrderflowRequest")
                .output_type("crate::orderflow::OrderflowResponse")
                .client_streaming()
                .server_streaming()
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("dropcopy")
                .route_name("Dropcopy")
                .input_type("crate::orderflow::DropcopyRequest")
                .output_type("crate::orderflow::DropcopyResponse")
                .server_streaming()
                .codec_path(json_codec)
                .build(),
        )
        .build();
    let json_oms_service = tonic_build::manual::Service::builder()
        .name("Oms")
        .package("json.architect")
        .method(
            tonic_build::manual::Method::builder()
                .name("place_order")
                .route_name("PlaceOrder")
                .input_type("crate::oms::PlaceOrderRequest")
                .output_type("crate::orderflow::Order")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("cancel_order")
                .route_name("CancelOrder")
                .input_type("crate::oms::CancelOrderRequest")
                .output_type("crate::orderflow::Cancel")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("cancel_all_orders")
                .route_name("CancelAllOrders")
                .input_type("crate::oms::CancelAllOrdersRequest")
                .output_type("crate::oms::CancelAllOrdersResponse")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("open_orders")
                .route_name("OpenOrders")
                .input_type("crate::oms::OpenOrdersRequest")
                .output_type("crate::oms::OpenOrdersResponse")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("pending_cancels")
                .route_name("PendingCancels")
                .input_type("crate::oms::PendingCancelsRequest")
                .output_type("crate::oms::PendingCancelsResponse")
                .codec_path(json_codec)
                .build(),
        )
        .build();
    let json_folio_service = tonic_build::manual::Service::builder()
        .name("Folio")
        .package("json.architect")
        .method(
            tonic_build::manual::Method::builder()
                .name("account_summary")
                .route_name("AccountSummary")
                .input_type("crate::folio::AccountSummaryRequest")
                .output_type("crate::folio::AccountSummary")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("account_summaries")
                .route_name("AccountSummaries")
                .input_type("crate::folio::AccountSummariesRequest")
                .output_type("crate::folio::AccountSummariesResponse")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("account_history")
                .route_name("AccountHistory")
                .input_type("crate::folio::AccountHistoryRequest")
                .output_type("crate::folio::AccountHistoryResponse")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("historical_fills")
                .route_name("HistoricalFills")
                .input_type("crate::folio::HistoricalFillsRequest")
                .output_type("crate::folio::HistoricalFillsResponse")
                .codec_path(json_codec)
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("historical_orders")
                .route_name("HistoricalOrders")
                .input_type("crate::folio::HistoricalOrdersRequest")
                .output_type("crate::folio::HistoricalOrdersResponse")
                .codec_path(json_codec)
                .build(),
        )
        .build();
    let mut schema_gen_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    schema_gen_dir.push("schema/generated");
    schema_builder::manual::Builder::new()
        .rewrite_crate("architect_api")
        .out_dir(schema_gen_dir)
        .emit_composite_package(true)
        .compile(&[
            &json_health_service,
            &json_symbology_service,
            &json_marketdata_service,
            &json_marketdata_snapshots_service,
            &json_orderflow_service,
            &json_oms_service,
            &json_folio_service,
        ]);
    tonic_build::manual::Builder::new().out_dir("schema/generated").compile(&[
        json_health_service,
        json_symbology_service,
        json_marketdata_service,
        json_marketdata_snapshots_service,
        json_orderflow_service,
        json_oms_service,
        json_folio_service,
    ]);
}

fn main() {
    #[cfg(feature = "tonic")]
    build_grpc_stubs();
}
