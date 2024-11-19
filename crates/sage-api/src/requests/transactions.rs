use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{Amount, CoinSpendJson, SpendBundleJson, TransactionSummary};

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct SendXch {
    pub address: String,
    pub amount: Amount,
    pub fee: Amount,
    #[serde(default)]
    pub auto_submit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct CombineXch {
    pub coin_ids: Vec<String>,
    pub fee: Amount,
    #[serde(default)]
    pub auto_submit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct SplitXch {
    pub coin_ids: Vec<String>,
    pub output_count: u32,
    pub fee: Amount,
    #[serde(default)]
    pub auto_submit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct CombineCat {
    pub coin_ids: Vec<String>,
    pub fee: Amount,
    #[serde(default)]
    pub auto_submit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct SplitCat {
    pub coin_ids: Vec<String>,
    pub output_count: u32,
    pub fee: Amount,
    #[serde(default)]
    pub auto_submit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct IssueCat {
    pub name: String,
    pub ticker: String,
    pub amount: Amount,
    pub fee: Amount,
    #[serde(default)]
    pub auto_submit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct SendCat {
    pub asset_id: String,
    pub address: String,
    pub amount: Amount,
    pub fee: Amount,
    #[serde(default)]
    pub auto_submit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct CreateDid {
    pub name: String,
    pub fee: Amount,
    #[serde(default)]
    pub auto_submit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct BulkMintNfts {
    pub mints: Vec<NftMint>,
    pub did_id: String,
    pub fee: Amount,
    #[serde(default)]
    pub auto_submit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct NftMint {
    pub edition_number: Option<u32>,
    pub edition_total: Option<u32>,
    pub data_uris: Vec<String>,
    pub metadata_uris: Vec<String>,
    pub license_uris: Vec<String>,
    pub royalty_address: Option<String>,
    pub royalty_percent: Amount,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct TransferNfts {
    pub nft_ids: Vec<String>,
    pub address: String,
    pub fee: Amount,
    #[serde(default)]
    pub auto_submit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct AddNftUri {
    pub nft_id: String,
    pub uri: String,
    pub fee: Amount,
    pub kind: NftUriKind,
    #[serde(default)]
    pub auto_submit: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum NftUriKind {
    Data,
    Metadata,
    License,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct AssignNftsToDid {
    pub nft_ids: Vec<String>,
    pub did_id: Option<String>,
    pub fee: Amount,
    #[serde(default)]
    pub auto_submit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct TransferDids {
    pub did_ids: Vec<String>,
    pub address: String,
    pub fee: Amount,
    #[serde(default)]
    pub auto_submit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct SignCoinSpends {
    pub coin_spends: Vec<CoinSpendJson>,
    #[serde(default)]
    pub auto_submit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct SignCoinSpendsResponse {
    pub spend_bundle: SpendBundleJson,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct SubmitTransaction {
    pub spend_bundle: SpendBundleJson,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
pub struct SubmitTransactionResponse {}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct TransactionResponse {
    pub summary: TransactionSummary,
    pub coin_spends: Vec<CoinSpendJson>,
}