use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct CoinInfo {
    pub name: String,
    pub symbol: String,
    pub max_supply: Option<Decimal>,
    pub circulating_supply: Option<Decimal>,
    pub total_supply: Option<Decimal>,
    pub infinite_supply: bool,
    pub price: Option<Decimal>,
    pub volume_24h: Option<Decimal>,
    pub volume_change_24h: Option<Decimal>,
    pub percent_change_1h: Option<Decimal>,
    pub percent_change_24h: Option<Decimal>,
    pub percent_change_7d: Option<Decimal>,
    pub percent_change_30d: Option<Decimal>,
    pub percent_change_60d: Option<Decimal>,
    pub percent_change_90d: Option<Decimal>,
    pub market_cap: Option<Decimal>,
    pub fully_diluted_market_cap: Option<Decimal>,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLEnum))]
pub enum CmeSecurityType {
    CASH,
    COMBO,
    FRA,
    FUT,
    FWD,
    IDX,
    INDEX,
    IRS,
    OOC,
    OOF,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "juniper", derive(juniper::GraphQLObject))]
pub struct CmeProductGroupInfo {
    pub product_guid: Option<String>,
    pub product_name: Option<String>,
    pub security_type: CmeSecurityType,
    pub clearing_symbol: Option<String>,
    pub master_symbol: Option<String>,
    pub exchange_clearing: Option<String>,
    pub exchange_globex: Option<String>,
    pub is_derived_block_eligible: Option<String>,
    pub asset_class: Option<String>,
    pub asset_sub_class: Option<String>,
    pub sector: Option<String>,
    pub sub_sector: Option<String>,
    pub is_tas_product: Option<String>,
    pub is_btic_product: Option<String>,
    pub is_tam_product: Option<String>,
    pub rfq_cross_eligible: Option<String>,
    pub mass_quote_eligible: Option<String>,
    pub daily_flag: Option<String>,
    pub settle_px_ccy: Option<String>,
    pub efr_eligible: Option<String>,
    pub floor_put_symbol: Option<String>,
    pub block_trade_eligible: Option<String>,
    pub efp_eligible: Option<String>,
    pub flex_eligible: Option<String>,
    pub px_unit_of_measure_qty: Option<i32>,
    pub negative_strike_eligible: Option<String>,
    pub min_globex_ord_qty: Option<String>,
    pub max_globex_ord_qty: Option<String>,
    pub negative_px_eligible: Option<String>,
    pub px_unit_of_measure: Option<String>,
    pub trade_px_ccy: Option<String>,
    pub floor_call_symbol: Option<String>,
    pub ebf_eligible: Option<String>,
    pub fractional: Option<String>,
    pub globex_group_code: Option<String>,
    pub itc_code: Option<String>,
    pub price_band: Option<String>,
    pub otc_eligible: Option<String>,
    pub globex_gt_eligible: Option<String>,
    pub px_quote_method: Option<String>,
    pub last_updated: Option<String>,
    pub market_segment_id: Option<i32>,
    pub globex_match_algo: Option<String>,
    pub ilink_eligible: Option<String>,
    pub clearport_eligible: Option<String>,
    pub unit_of_measure: Option<String>,
    pub globex_eligible: Option<String>,
    pub floor_eligible: Option<String>,
    pub variable_qty_flag: Option<String>,
    pub strategy_type: Option<String>,
    pub assignment_method: Option<String>,
    pub price_multiplier: Option<Decimal>,
    pub main_fraction: Option<i32>,
    pub unit_of_measure_qty: Option<Decimal>,
    pub globex_product_code: Option<String>,
    pub default_min_tick: Option<String>,
    pub reduced_tick_notes: Option<String>,
    pub min_qtrly_serial_tick: Option<String>,
    pub min_outright_tick: Option<String>,
    pub minimum_tick_note: Option<String>,
    pub min_clearport_tick: Option<String>,
    pub min_cabinet_tick_rules: Option<String>,
    pub minimum_half_tick: Option<String>,
    pub globex_min_tick: Option<String>,
    pub min_clearport_floor_tick: Option<String>,
    pub midcurve_tick_rules: Option<String>,
    pub calendar_tick_rules: Option<String>,
    pub floor_schedule: Option<String>,
    pub std_trading_hours: Option<String>,
    pub globex_schedule: Option<String>,
    pub clearport_schedule: Option<String>,
    pub tot_ltd: Option<String>,
    pub tot_midcurve: Option<String>,
    pub tot_quarterly: Option<String>,
    pub tot_serial: Option<String>,
    pub tot_clearport: Option<String>,
    pub tot_globex: Option<String>,
    pub tot_default: Option<String>,
    pub tot_floor: Option<String>,
    pub serial_listing_rules: Option<String>,
    pub regular_listing_rules: Option<String>,
    pub quarterly_listing_rules: Option<String>,
    pub floor_listing_rules: Option<String>,
    pub midcurve_options_rules: Option<String>,
    pub default_listing_rules: Option<String>,
    pub globex_listing_rules: Option<String>,
    pub last_delivery_rules: Option<String>,
    pub commodity_standards: Option<String>,
    pub settle_method: Option<String>,
    pub marker_stlmt_rules: Option<String>,
    pub limit_rules: Option<String>,
    pub days_or_hours: Option<String>,
    pub reportable_positions: Option<String>,
    pub price_quotation: Option<String>,
    pub settlement_procedure: Option<String>,
    pub exercise_style: Option<String>,
    pub settlement_at_expiration: Option<String>,
    pub strike_price_interval: Option<String>,
    pub mdp3_channel: Option<String>,
    pub globex_display_factor: Option<String>,
    pub var_cab_px_high: Option<String>,
    pub var_cab_px_low: Option<String>,
    pub clearing_cab_px: Option<String>,
    pub globex_cab_px: Option<String>,
    pub is_synthetic_product: Option<String>,
    pub itm_otm: Option<String>,
    pub contrary_instructions_allowed: Option<String>,
    pub settle_using_fixing_px: Option<String>,
    pub settlement_type: Option<String>,
    pub opt_style: Option<String>,
    pub trading_cut_off_time: Option<String>,
    pub is_taco_product: Option<String>,
    pub exercise_style_american_european: Option<String>,
    pub is_tmac_product: Option<String>,
    pub clearing_org_id: Option<String>,
    pub gc_basket_identifier: Option<String>,
    pub globex_group_descr: Option<String>,
    pub min_incremental_order: Option<String>,
    pub par_or_money: Option<String>,
    pub repo_year_days: Option<String>,
    pub subtype: Option<String>,
    pub valuation_method: Option<String>,
    pub contract_notional_amount: Option<Decimal>,
    pub rbt_eligible_ind: Option<String>,
    pub dirty_price_tick: Option<String>,
    pub dirty_price_rounding: Option<String>,
    pub min_days_to_mat: Option<String>,
    pub spread_pricing_convention: Option<String>,
    pub is_pm_eligible: Option<String>,
    pub fixing_time_zone: Option<String>,
    pub settlement_locale: Option<String>,
    pub trade_close_off_set: Option<String>,
    pub on_sef: Option<String>,
    pub price_precision: Option<String>,
    pub on_mtf: Option<String>,
    pub good_for_session: Option<String>,
    pub fixing_source: Option<String>,
    pub settl_ccy: Option<String>,
    pub is_efix_product: Option<String>,
    pub market_data: Option<String>,
    pub fixed_payout: Option<f64>,
    pub size_priority_qty: Option<String>,
    pub top_eligible: Option<String>,
    pub alt_globex_min_tick: Option<String>,
    pub alt_globex_tick_constraint: Option<String>,
    pub max_bid_ask_constraint: Option<String>,
    pub alt_min_quote_life: Option<String>,
    pub subfraction: Option<String>,
    pub url: Option<String>,
    pub category: Option<String>,
    pub sub_category: Option<String>,
}
