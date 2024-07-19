#![cfg(feature = "netidx")]

use crate::{
    folio::FolioMessage,
    orderflow::{
        AberrantFill, Ack, Cancel, CancelAll, Fill, Order, OrderflowMessage, Out, Reject,
        RejectReason,
    },
    symbology::{market::NormalizedMarketInfo, MarketId},
    AccountPermissions, OrderId, UserId,
};
use arcstr::ArcStr;
use chrono::{DateTime, Utc};
use derive::FromValue;
use netidx_derive::Pack;
use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Deref,
    sync::Arc,
};

#[derive(Debug, Clone, Serialize, Deserialize, Pack)]
pub struct CqgMarketInfo {
    pub deleted: bool,
    // Don't store contract_id because it's session-specific
    pub symbol_id: Option<String>,
    pub description: String,
    pub cfi_code: Option<String>,
    pub cqg_contract_symbol: Option<String>,
    pub correct_price_scale: Decimal,
    pub exchange_symbol: String,
    pub exchange_group_symbol: String,
    pub tick_size: Decimal,  // uncorrected
    pub tick_value: Decimal, // uncorrected
    pub trade_size_increment: Decimal,
    pub market_data_delay_ms: i64,
    pub initial_margin: Option<Decimal>,
    pub maintenance_margin: Option<Decimal>,
    pub last_trading_date: DateTime<Utc>,
    pub first_notice_date: Option<DateTime<Utc>>,
}

impl NormalizedMarketInfo for CqgMarketInfo {
    fn tick_size(&self) -> Decimal {
        self.tick_size
    }

    fn step_size(&self) -> Decimal {
        self.trade_size_increment
    }

    fn is_delisted(&self) -> bool {
        self.deleted
    }

    fn initial_margin(&self) -> Option<Decimal> {
        self.initial_margin
    }

    fn maintenance_margin(&self) -> Option<Decimal> {
        self.maintenance_margin
    }
}

impl std::fmt::Display for CqgMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Pack, Serialize, Deserialize)]
pub struct CqgOrder {
    #[serde(flatten)]
    pub order: Order,
}

impl Deref for CqgOrder {
    type Target = Order;

    fn deref(&self) -> &Self::Target {
        &self.order
    }
}

#[derive(Debug, Clone, Pack, Serialize, Deserialize)]
pub struct CqgTrade {
    pub order_id: OrderId,
    pub exec_id: String,
    pub scaled_price: i64,
    pub qty: Decimal,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct CqgAccountSummary {
    /// True if this is a snapshot related message.
    /// Since snapshot might be sent in several messages (including none), client should use
    /// TradeSnapshotCompletion message as an indicator of complete snapshot delivery for a particular subscription.
    pub is_snapshot: Option<bool>,

    /// Account id of this status.
    /// It is required field.
    pub account_id: Option<i32>,

    /// Currency code of account values (ISO 4217 based).
    /// It is required field in snapshot and included into updates only if changed.
    pub currency: Option<String>,

    /// Identifiers of fields being cleared.
    /// E.g. to clear total_margin server will include value 6 into the collection.
    pub cleared_fields: Vec<u32>,

    /// Margin requirement calculated for worst-case based on open positions and working orders.
    pub total_margin: Option<f64>,

    /// Margin requirement based on current positions only.
    pub position_margin: Option<f64>,

    /// Available account funds including balance, realized profit (or loss), collateral and credits.
    /// OTE and MVO are included regarding the account risk parameters.
    /// For a group account, purchasing power is a recent snapshot calculated by the server.
    /// It uses data from all accounts in the group, so it will not be synchronized with values
    /// reported for only this account. Also, for group accounts, OTE and MVO components of
    /// purchasing power will not be synchronized with market data updates.
    /// See trading_account_2.Account.is_group_member.
    pub purchasing_power: Option<f64>,

    /// Open trade equity, or potential profit (or loss) from futures and future-style options positions
    /// based on opening price of the position and the current future trade/best bid/best ask
    /// (regarding to the risk account settings) or settlement price if trade is not available.
    pub ote: Option<f64>,

    /// Market value of options calculated as the current market trade/best bid/best ask of the option
    /// (regarding to the risk account settings) times the number of options
    /// (positive for long options and negative for short options) in the portfolio.
    pub mvo: Option<f64>,

    /// Market value of futures calculated as the current market trade/best bid/best ask
    /// (regarding to the risk account settings) times the number of futures
    /// (positive for long and negative for short) in the portfolio.
    pub mvf: Option<f64>,

    /// Allowable margin credit of the account.
    pub margin_credit: Option<f64>,

    /// Cash Excess.
    pub cash_excess: Option<f64>,

    /// Current account's balance. In particular includes: yesterday balance, profit/loss, option premium,
    /// commission and Forex instrument positions.
    pub current_balance: Option<f64>,

    /// Realized profit/loss.
    pub profit_loss: Option<f64>,

    /// Unrealized profit/loss for options.
    pub unrealized_profit_loss: Option<f64>,

    /// Cash balance from the last statement.
    pub yesterday_balance: Option<f64>,

    /// Open trade equity for futures and futures-style options from the last statement.
    pub yesterday_ote: Option<f64>,

    /// Market value of premium-style options and fixed income from the last statement.
    pub yesterday_mvo: Option<f64>,

    /// Collateral on deposit.
    pub yesterday_collateral: Option<f64>,

    /// (profit_loss / abs(yesterday_balance)) in percentage.
    pub net_change_pc: Option<f64>,

    /// Sum of all fill sizes for the current day.
    pub total_filled_qty: Option<Decimal>,

    /// Count of filled orders for the current day.
    pub total_filled_orders: Option<u32>,

    /// Sum of position quantities among all long open positions on the account.
    pub long_open_positions_qty: Option<Decimal>,

    /// Sum of position quantities among all short open positions on the account.
    pub short_open_positions_qty: Option<Decimal>,

    /// Minimal value of days till contract expiration (in calendar days, not trading) among
    /// all open positions on the account.
    /// Not set if there are no open positions on the account.
    pub min_days_till_position_contract_expiration: Option<u32>,

    /// Limit of the maximum value of purchasing power for the account.
    /// Can be empty e.g. when the account is a group account member.
    /// See trading_account_2.Account.is_group_member.
    pub purchasing_power_limit: Option<f64>,
}

impl CqgAccountSummary {
    pub fn merge(&mut self, other: &CqgAccountSummary) {
        let CqgAccountSummary {
            is_snapshot: _,
            account_id,
            currency,
            cleared_fields,
            total_margin,
            position_margin,
            purchasing_power,
            ote,
            mvo,
            mvf,
            margin_credit,
            cash_excess,
            current_balance,
            profit_loss,
            unrealized_profit_loss,
            yesterday_balance,
            yesterday_ote,
            yesterday_mvo,
            yesterday_collateral,
            net_change_pc,
            total_filled_qty,
            total_filled_orders,
            long_open_positions_qty,
            short_open_positions_qty,
            min_days_till_position_contract_expiration,
            purchasing_power_limit,
        } = other;
        self.account_id = account_id.or(self.account_id);
        self.currency = currency.clone().or_else(|| self.currency.clone());
        self.total_margin = total_margin.or(self.total_margin);
        self.position_margin = position_margin.or(self.position_margin);
        self.purchasing_power = purchasing_power.or(self.purchasing_power);
        self.ote = ote.or(self.ote);
        self.mvo = mvo.or(self.mvo);
        self.mvf = mvf.or(self.mvf);
        self.margin_credit = margin_credit.or(self.margin_credit);
        self.cash_excess = cash_excess.or(self.cash_excess);
        self.current_balance = current_balance.or(self.current_balance);
        self.profit_loss = profit_loss.or(self.profit_loss);
        self.unrealized_profit_loss =
            unrealized_profit_loss.or(self.unrealized_profit_loss);
        self.yesterday_balance = yesterday_balance.or(self.yesterday_balance);
        self.yesterday_ote = yesterday_ote.or(self.yesterday_ote);
        self.yesterday_mvo = yesterday_mvo.or(self.yesterday_mvo);
        self.yesterday_collateral = yesterday_collateral.or(self.yesterday_collateral);
        self.net_change_pc = net_change_pc.or(self.net_change_pc);
        self.total_filled_qty = total_filled_qty.or(self.total_filled_qty);
        self.total_filled_orders = total_filled_orders.or(self.total_filled_orders);
        self.long_open_positions_qty =
            long_open_positions_qty.or(self.long_open_positions_qty);
        self.short_open_positions_qty =
            short_open_positions_qty.or(self.short_open_positions_qty);
        self.min_days_till_position_contract_expiration =
            min_days_till_position_contract_expiration
                .or(self.min_days_till_position_contract_expiration);
        self.purchasing_power_limit =
            purchasing_power_limit.or(self.purchasing_power_limit);
        for field in cleared_fields {
            match field {
                2 => {
                    self.is_snapshot = None;
                }
                3 => {
                    self.account_id = None;
                }
                4 => {
                    self.currency = None;
                }
                6 => {
                    self.total_margin = None;
                }
                7 => {
                    self.position_margin = None;
                }
                8 => {
                    self.purchasing_power = None;
                }
                9 => {
                    self.ote = None;
                }
                10 => {
                    self.mvo = None;
                }
                11 => {
                    self.mvf = None;
                }
                12 => {
                    self.margin_credit = None;
                }
                13 => {
                    self.cash_excess = None;
                }
                15 => {
                    self.current_balance = None;
                }
                16 => {
                    self.profit_loss = None;
                }
                17 => {
                    self.unrealized_profit_loss = None;
                }
                18 => {
                    self.yesterday_balance = None;
                }
                24 => {
                    self.yesterday_ote = None;
                }
                25 => {
                    self.yesterday_mvo = None;
                }
                14 => {
                    self.yesterday_collateral = None;
                }
                26 => {
                    self.net_change_pc = None;
                }
                19 => {
                    self.total_filled_qty = None;
                }
                20 => {
                    self.total_filled_orders = None;
                }
                21 => {
                    self.long_open_positions_qty = None;
                }
                22 => {
                    self.short_open_positions_qty = None;
                }
                23 => {
                    self.min_days_till_position_contract_expiration = None;
                }
                27 => {
                    self.purchasing_power_limit = None;
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct CqgPositionStatus {
    pub account: i32,
    pub market: MarketId,
    pub positions: Vec<CqgPosition>,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize, PartialEq, Eq)]
pub struct CqgPosition {
    /// Surrogate id as a key for updates.
    pub id: i32,

    /// Position size, zero means that this position is deleted.
    /// Note: quantity can be safely compared to zero, because this is an integral number of
    /// ContractMetadata.volume_scale units.
    pub qty: Option<Decimal>,

    /// Position average price.
    /// NOTE: Since it could be an aggregated position price is sent in correct format directly.
    pub price_correct: Decimal,

    /// Exchange specific trade date when the position was open or last changed (date only value).
    pub trade_date: i64,

    /// Statement date (date value only).
    pub statement_date: i64,

    /// UTC trade time (including date) if available, it might not be available e.g. for the previous day positions.
    pub trade_utc_timestamp: Option<DateTime<Utc>>,

    /// True if the price is an aggregated position price.
    pub is_aggregated: bool,

    /// True if the open position is short (result of a sell operation), long otherwise.
    /// Undefined for deleted position (qty is 0).
    pub is_short: bool,

    /// Whether it is a yesterday or a today position.
    /// NOTE: where available, this attribute is from the exchange trade date perspective. It is used for
    /// position tracking and open/close instructions. It is not the same as previous day (associated
    /// with brokerage statement) vs. intraday. It is also not static. For example, an intraday fill
    /// with open_close_effect=OPEN will appear, when it is received during the trading session, in an open
    /// position or matched trade with is_yesterday=false. After the exchange trade date rolls over for
    /// that contract, and before the brokerage statement arrives reflecting it as a previous day position,
    /// the same open position or matched trade will contain is_yesterday=true.
    pub is_yesterday: Option<bool>,

    /// Speculation type of the position. One of SpeculationType enum.
    pub speculation_type: Option<u32>,
}

impl PartialOrd for CqgPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for CqgPosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub struct CancelReject {
    pub cancel_id: ArcStr,
    pub order_id: OrderId,
    pub reason: RejectReason,
}

#[derive(
    Clone,
    Debug,
    FromValue,
    Serialize,
    Deserialize,
    Pack,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct CqgAccount {
    pub user_id: UserId,
    pub user_email: String,
    pub clearing_venue: String,
    pub cqg_account_id: i32,
    pub cqg_trader_id: String,
}

pub type AccountProxyConfig = BTreeMap<UserId, AccountProxy>;
#[derive(Deserialize, Serialize, Clone, Pack, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AccountProxySelector {
    AccountId { cqg_account_id: i32 },
    AccountIds { cqg_account_ids: Vec<i32> },
    TraderId { cqg_trader_id: String },
    TraderIds { cqg_trader_ids: Vec<String> },
    AllAccounts,
    AllAccountsForFCMs { clearing_venues: Vec<String> },
}

#[derive(Debug, postgres_types::FromSql)]
#[postgres(name = "Selector")]
pub enum AccountProxySelectorType {
    #[postgres(name = "ACCOUNTS")]
    Account,
    #[postgres(name = "TRADERS")]
    Trader,
    #[postgres(name = "ALL")]
    All,
    #[postgres(name = "FCM")]
    Fcm,
}

impl AccountProxySelector {
    pub fn selects(&self, cqg_account: &CqgAccount) -> bool {
        match self {
            AccountProxySelector::AccountId { cqg_account_id } => {
                cqg_account.cqg_account_id == *cqg_account_id
            }
            AccountProxySelector::AccountIds { cqg_account_ids } => {
                cqg_account_ids.contains(&cqg_account.cqg_account_id)
            }
            AccountProxySelector::TraderId { cqg_trader_id } => {
                &cqg_account.cqg_trader_id == cqg_trader_id
            }
            AccountProxySelector::TraderIds { cqg_trader_ids } => {
                cqg_trader_ids.contains(&cqg_account.cqg_trader_id)
            }
            AccountProxySelector::AllAccounts => true,
            AccountProxySelector::AllAccountsForFCMs { clearing_venues } => {
                clearing_venues.contains(&cqg_account.clearing_venue)
            }
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Pack, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountProxy {
    pub selector: AccountProxySelector,
    #[serde(flatten)]
    pub permissions: AccountPermissions,
}

#[derive(Debug, Clone, Pack, FromValue, Serialize, Deserialize)]
pub enum CqgMessage {
    Order(CqgOrder),
    Cancel(Cancel),
    CancelAll,
    Ack(Ack),
    Out(Out),
    Fill(Result<Fill, AberrantFill>),
    Reject(Reject),
    CancelReject(CancelReject),
    Folio(FolioMessage),
    UpdateCqgAccounts {
        accounts: Arc<BTreeSet<CqgAccount>>,
        account_proxies: Arc<AccountProxyConfig>,
        is_snapshot: bool,
    },
    CqgTrades(Vec<CqgTrade>),
    CqgAccountSummary(CqgAccountSummary),
    CqgPositionStatus(CqgPositionStatus),
    UpdateCqgAccountsFromDb,
}

impl TryInto<OrderflowMessage> for &CqgMessage {
    type Error = ();

    fn try_into(self) -> Result<OrderflowMessage, ()> {
        match self {
            CqgMessage::Order(o) => Ok(OrderflowMessage::Order(**o)),
            CqgMessage::Cancel(c) => Ok(OrderflowMessage::Cancel(*c)),
            CqgMessage::CancelAll => {
                Ok(OrderflowMessage::CancelAll(CancelAll { venue_id: None }))
            }
            CqgMessage::Ack(a) => Ok(OrderflowMessage::Ack(*a)),
            CqgMessage::Out(o) => Ok(OrderflowMessage::Out(*o)),
            CqgMessage::Fill(f) => Ok(OrderflowMessage::Fill(*f)),
            CqgMessage::Reject(r) => Ok(OrderflowMessage::Reject(r.clone())),
            CqgMessage::CancelReject(_)
            | CqgMessage::UpdateCqgAccountsFromDb
            | CqgMessage::UpdateCqgAccounts { .. }
            | CqgMessage::Folio(_)
            | CqgMessage::CqgTrades(_)
            | CqgMessage::CqgAccountSummary(_)
            | CqgMessage::CqgPositionStatus(_) => Err(()),
        }
    }
}

impl TryInto<CqgMessage> for &OrderflowMessage {
    type Error = ();

    fn try_into(self) -> Result<CqgMessage, ()> {
        match self {
            OrderflowMessage::Order(o) => Ok(CqgMessage::Order(CqgOrder { order: *o })),
            OrderflowMessage::Cancel(c) => Ok(CqgMessage::Cancel(*c)),
            OrderflowMessage::CancelAll(_) => Ok(CqgMessage::CancelAll),
            OrderflowMessage::Ack(a) => Ok(CqgMessage::Ack(*a)),
            OrderflowMessage::Out(o) => Ok(CqgMessage::Out(*o)),
            OrderflowMessage::Reject(r) => Ok(CqgMessage::Reject(r.clone())),
            OrderflowMessage::Fill(f) => Ok(CqgMessage::Fill(*f)),
        }
    }
}

impl TryInto<FolioMessage> for &CqgMessage {
    type Error = ();

    fn try_into(self) -> Result<FolioMessage, ()> {
        match self {
            CqgMessage::Folio(f) => Ok(f.clone()),
            _ => Err(()),
        }
    }
}

impl TryFrom<&FolioMessage> for CqgMessage {
    type Error = ();

    fn try_from(f: &FolioMessage) -> Result<Self, ()> {
        Ok(Self::Folio(f.clone()))
    }
}
