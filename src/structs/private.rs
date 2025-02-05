use super::reqs::OrderStop;
use super::DateTime;
use crate::utils::{option_datetime_with_tz_from_string, datetime_with_tz_from_string, datetime_from_string, f64_from_string, f64_opt_from_string, usize_from_string};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

// Private

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub id: Uuid,
    pub currency: String,
    #[serde(deserialize_with = "f64_from_string")]
    pub balance: f64,
    #[serde(deserialize_with = "f64_from_string")]
    pub available: f64,
    #[serde(deserialize_with = "f64_from_string")]
    pub hold: f64,
    pub profile_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountHistory {
    #[serde(deserialize_with = "usize_from_string")]
    pub id: usize,
    pub created_at: DateTime,
    #[serde(deserialize_with = "f64_from_string")]
    pub amount: f64,
    #[serde(deserialize_with = "f64_from_string")]
    pub balance: f64,
    #[serde(skip_deserializing)]
    pub _type: AccountHistoryType,
    #[serde(flatten)]
    pub details: AccountHistoryDetails, // variants are not not clear
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum AccountHistoryType {
    Fee,
    Match,
    Rebate,
    Transfer,
    Conversion,
    NotSet,
}

impl Default for AccountHistoryType {
    fn default() -> Self {
        AccountHistoryType::NotSet
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "details")]
#[serde(rename_all = "camelCase")]
pub enum AccountHistoryDetails {
    Fee {
        order_id: Uuid,
        product_id: String,
        #[serde(deserialize_with = "usize_from_string")]
        trade_id: usize,
    },
    Match {
        order_id: Uuid,
        product_id: String,
        #[serde(deserialize_with = "usize_from_string")]
        trade_id: usize,
    },
    Rebate {
        order_id: Uuid,
        product_id: String,
        #[serde(deserialize_with = "usize_from_string")]
        trade_id: usize,
    },
    Transfer {
        transfer_id: Uuid,
        transfer_type: AccountHistoryDetailsTransferType,
    },
    Conversion {
        conversion_id: Uuid,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum AccountHistoryDetailsTransferType {
    Deposit,
    Withdraw,
}

impl<'a> From<&'a AccountHistoryDetails> for AccountHistoryType {
    fn from(item: &'a AccountHistoryDetails) -> Self {
        match item {
            AccountHistoryDetails::Fee { .. } => AccountHistoryType::Fee,
            AccountHistoryDetails::Match { .. } => AccountHistoryType::Match,
            AccountHistoryDetails::Transfer { .. } => AccountHistoryType::Transfer,
            AccountHistoryDetails::Rebate { .. } => AccountHistoryType::Rebate,
            AccountHistoryDetails::Conversion { .. } => AccountHistoryType::Conversion,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountHolds {
    pub id: Uuid,
    pub account_id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub amount: f64,
    #[serde(rename = "type")]
    pub _type: AccountHoldsType,
    #[serde(rename = "ref")]
    pub _ref: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum AccountHoldsType {
    Order,
    Transfer,
}

// limit:{"id":"e9d0ff7a-ed50-4040-87a7-c884ae562807","price":"1.12000000","size":"1.00000000","product_id":"BTC-USD","side":"buy","stp":"dc","type":"limit","time_in_force":"GTC","post_only":true,"created_at":"2018-08-23T18:53:42.144811Z","fill_fees":"0.0000000000000000","filled_size":"0.00000000","executed_value":"0.0000000000000000","status":"pending","settled":false}
// market:{"id":"ea565dc3-1656-49d7-bcdb-d99981ce35a7","size":"0.00100000","product_id":"BTC-USD","side":"buy","stp":"dc","funds":"28.2449436100000000","type":"market","post_only":false,"created_at":"2018-08-23T18:43:18.964413Z","fill_fees":"0.0000000000000000","filled_size":"0.00000000","executed_value":"0.0000000000000000","status":"pending","settled":false}
// call:[{"id":"063da13d-6aba-45e1-91ca-89f8514da989","price":"100000.00000000","size":"0.00100000","product_id":"BTC-USD","side":"sell","type":"limit","time_in_force":"GTC","post_only":true,"created_at":"2018-08-24T04:50:01.139098Z","fill_fees":"0.0000000000000000","filled_size":"0.00000000","executed_value":"0.0000000000000000","status":"open","settled":false}]

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    pub id: Uuid,
    pub product_id: String,
    pub side: super::reqs::OrderSide,
    pub stp: Option<String>, // Options because its not in get_orders, but in set_order
    #[serde(default)]
    #[serde(deserialize_with = "f64_opt_from_string")]
    pub funds: Option<f64>,
    #[serde(default)]
    #[serde(deserialize_with = "f64_opt_from_string")]
    pub specified_funds: Option<f64>,
    #[serde(flatten)]
    pub _type: OrderType,
    pub post_only: bool,
    pub created_at: DateTime,
    pub done_at: Option<DateTime>,
    pub done_reason: Option<String>,
    #[serde(deserialize_with = "f64_from_string")]
    pub fill_fees: f64,
    #[serde(deserialize_with = "f64_from_string")]
    pub filled_size: f64,
    #[serde(deserialize_with = "f64_from_string")]
    pub executed_value: f64,
    pub status: OrderStatus,
    pub settled: bool,
    #[serde(flatten)]
    pub stop: Option<OrderStop>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum OrderType {
    Limit {
        #[serde(deserialize_with = "f64_from_string")]
        size: f64,
        #[serde(deserialize_with = "f64_from_string")]
        price: f64,
        #[serde(flatten)]
        time_in_force: OrderTimeInForce,
    },
    Market {
        #[serde(default)]
        #[serde(deserialize_with = "f64_from_string")]
        size: f64,
        //        #[serde(deserialize_with = "f64_opt_from_string")]
        //        funds: Option<f64>
        #[serde(default)]
        #[serde(deserialize_with = "f64_from_string")]
        funds: f64,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "time_in_force")]
pub enum OrderTimeInForce {
    GTC,
    GTT {
        #[serde(deserialize_with = "datetime_from_string")]
        expire_time: DateTime,
    },
    IOC,
    FOK,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum OrderStatus {
    Open,
    Done,
    Pending,
    Active,
    Rejected,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = match self {
            OrderStatus::Open => "open",
            OrderStatus::Done => "done",
            OrderStatus::Pending => "pending",
            OrderStatus::Active => "active",
            OrderStatus::Rejected => "rejected",
        };
        write!(f, "{}", res)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fill {
    pub trade_id: usize,
    pub product_id: String,
    #[serde(deserialize_with = "f64_from_string")]
    pub price: f64,
    #[serde(deserialize_with = "f64_from_string")]
    pub size: f64,
    pub order_id: Uuid,
    pub created_at: DateTime,
    pub liquidity: FillLiquidity,
    #[serde(deserialize_with = "f64_from_string")]
    pub fee: f64,
    pub settled: bool,
    pub side: super::reqs::OrderSide,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FillLiquidity {
    M,
    T,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrailingVolume {
    pub product_id: String,
    #[serde(deserialize_with = "f64_from_string")]
    pub exchange_volume: f64,
    #[serde(deserialize_with = "f64_from_string")]
    pub volume: f64,
    pub recorded_at: DateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fees {
    #[serde(deserialize_with = "f64_from_string")]
    pub maker_fee_rate: f64,
    #[serde(deserialize_with = "f64_from_string")]
    pub taker_fee_rate: f64,
    #[serde(deserialize_with = "f64_from_string")]
    pub usd_volume: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transfer {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub _type: TransferType,
    #[serde(deserialize_with = "datetime_with_tz_from_string")]
    pub created_at: DateTime,
    #[serde(default)]
    #[serde(deserialize_with = "option_datetime_with_tz_from_string")]
    pub completed_at: Option<DateTime>,
    #[serde(default)]
    #[serde(deserialize_with = "option_datetime_with_tz_from_string")]
    pub canceled_at: Option<DateTime>,
    #[serde(deserialize_with = "option_datetime_with_tz_from_string")]
    pub processed_at: Option<DateTime>,
    pub account_id: Uuid,
    pub user_id: String,
    pub user_nonce: Option<String>,
    #[serde(deserialize_with = "f64_from_string")]
    pub amount: f64,
    #[serde(default)]
    pub currency: Option<String>,
    pub details: TransferDetails
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransferDetails {
    #[serde(default)]
    pub destination_tag: Option<String>,
    #[serde(default)]
    pub sent_to_address: Option<String>,
    #[serde(default)]
    pub coinbase_account_id: Option<String>,
    #[serde(default)]
    pub destination_tag_name: Option<String>,
    #[serde(default)]
    pub coinbase_withdrawal_id: Option<String>,
    #[serde(default)]
    pub coinbase_transaction_id: Option<String>,
    #[serde(default)]
    pub crypto_transaction_hash: Option<String>,
    #[serde(default)]
    pub coinbase_payment_method_id: Option<String>,
    #[serde(deserialize_with = "f64_opt_from_string")]
    #[serde(default)]
    pub fee: Option<f64>,
    #[serde(deserialize_with = "f64_opt_from_string")]
    #[serde(default)]
    pub subtotal: Option<f64>,
    #[serde(default)]
    pub crypto_address: Option<String>,
    #[serde(default)]
    pub crypto_transaction_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum TransferType {
    Deposit,
    Withdraw,
    InternalDeposit,
    InternalWithdraw
}

impl fmt::Display for TransferType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = match self {
            TransferType::Deposit => "deposit",
            TransferType::Withdraw => "withdraw",
            TransferType::InternalDeposit => "internal_deposit",
            TransferType::InternalWithdraw => "internal_withdraw"
        };
        write!(f, "{}", res)
    }
}