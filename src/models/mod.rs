use serde::{de, Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceSample {
    pub slot: u64,
    pub num_transactions: u64,
    pub num_slots: u64,
    pub sample_period_secs: u64,
    pub num_non_vote_transactions: u64,
}

#[derive(Debug, Deserialize)]
pub struct BlockTime {
    pub slot: u64,
    pub timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct VoteAccount {
    pub node_pubkey: String,
    pub vote_pubkey: String,
    pub activated_stake: u64,
    pub commission: u8,
    pub last_vote: u64,
    pub root_slot: u64,
}

#[derive(Debug, Deserialize)]
pub struct VoteAccounts {
    pub current: Vec<VoteAccount>,
    pub delinquent: Vec<VoteAccount>,
}

#[derive(Debug, Deserialize)]
pub struct Fees {
    pub blockhash: String,
    pub fee_calculator: FeeCalculator,
    pub last_valid_slot: u64,
}

#[derive(Debug, Deserialize)]
pub struct FeeCalculator {
    pub lamports_per_signature: u64,
}

#[derive(Debug, Deserialize)]
pub struct PriorityFee {
    pub slot: u64,
    pub prioritization_fee: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpochInfo {
    pub absolute_slot: u64,
    pub block_height: u64,
    pub epoch: u64,
    pub slot_index: u64,
    pub slots_in_epoch: u64,
    pub transaction_count: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAccountBalance {
    pub address: String,
    pub amount: String,
    pub decimals: u8,
    pub ui_amount: Option<f64>,
    pub ui_amount_string: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenSupplyValue {
    pub amount: String,
    pub decimals: u8,
    pub ui_amount: Option<f64>,
    pub ui_amount_string: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenSupply {
    pub context: Context,
    pub value: TokenSupplyValue,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub slot: u64,
}