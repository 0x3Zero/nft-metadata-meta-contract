#![allow(improper_ctypes)]

mod data;
mod defaults;
mod types;

use data::OpenSeaAttributes;
use ethabi::{decode, ParamType};
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;
use types::MetaContract;
use types::Metadata;
use types::NFTMetadataStandard;
use types::Transaction;
use types::{FinalMetadata, MetaContractResult};

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
}

#[marine]
pub fn on_execute(
    contract: MetaContract,
    metadatas: Vec<Metadata>,
    transaction: Transaction,
) -> MetaContractResult {
    let mut finals: Vec<FinalMetadata> = vec![];

    let data: Result<NFTMetadataStandard, serde_json::Error> =
        serde_json::from_str(&transaction.data);

    match data {
        Ok(metadata) => {}
        Err(_) => {
            return MetaContractResult {
                result: false,
                metadatas: Vec::new(),
                error_string: "Data is not a valid NFT metadata format.".to_string(),
            };
        }
    }

    finals.push(FinalMetadata {
        public_key: transaction.public_key.clone(),
        alias: "".to_string(),
        content: transaction.data,
        loose: 0,
        version: transaction.version,
    });

    MetaContractResult {
        result: true,
        metadatas: finals,
        error_string: "".to_string(),
    }
}

#[marine]
pub fn on_clone() -> bool {
    return false;
}

#[marine]
pub fn on_mint(
    contract: MetaContract,
    data_key: String,
    token_id: String,
    data: String,
) -> MetaContractResult {
    MetaContractResult {
        result: false,
        metadatas: vec![],
        error_string: "on_mint is not available".to_string(),
    }
}
