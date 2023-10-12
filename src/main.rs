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

    finals.push(FinalMetadata {
        public_key: transaction.public_key.clone(),
        alias: "".to_string(),
        content: transaction.data.clone(),
        loose: 0,
        version: transaction.data_key.clone(),
    });

    let exists_token = metadatas
        .iter()
        .any(|m| m.public_key == "0x01" && m.alias == "token" && m.version == transaction.data_key);

    let exists_lineage_key = metadatas.iter().any(|m| {
        m.public_key == "0x01" && m.alias == "lineage_key" && m.version == transaction.data_key
    });

    if !exists_token {
        let content_1 = format!(
            r#"{{ 
                "address": "{}", 
                "chain": "{}", 
                "id": "{}"
            }}"#,
            transaction.token_address, transaction.chain_id, transaction.token_id
        );

        finals.push(FinalMetadata {
            public_key: "0x01".to_string(),
            alias: "token".to_string(),
            content: content_1,
            loose: 0,
            version: transaction.data_key.clone(),
        });
    }

    if !exists_lineage_key {
        finals.push(FinalMetadata {
            public_key: "0x01".to_string(),
            alias: "lineage_key".to_string(),
            content: transaction.data_key.clone(),
            loose: 0,
            version: transaction.data_key.clone(),
        });
    }

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

#[marine]
pub fn on_post_mint(
    metadatas: Vec<FinalMetadata>,
    contract: MetaContract,
    data_key: String,
    token_address: String,
    chain: String,
    token_id: String,
) -> MetaContractResult {
    let content_1 = format!(
        r#"{{ 
            "address": "{}", 
            "chain": "{}", 
            "id": "{}"
        }}"#,
        token_address, chain, token_id
    );

    let metadata_1 = FinalMetadata {
        public_key: "0x01".to_string(),
        alias: "token".to_string(),
        content: content_1,
        loose: 0,
        version: data_key.clone(),
    };

    let metadata_2 = FinalMetadata {
        public_key: "0x01".to_string(),
        alias: "lineage_key".to_string(),
        content: data_key.clone(),
        loose: 0,
        version: data_key.clone(),
    };

    let mut new_metadatas = metadatas.clone();
    new_metadatas.push(metadata_1);
    new_metadatas.push(metadata_2);

    MetaContractResult {
        result: true,
        metadatas: new_metadatas,
        error_string: "".to_string(),
    }
}
