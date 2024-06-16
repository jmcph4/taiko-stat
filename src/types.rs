use std::time::{SystemTime, UNIX_EPOCH};

use alloy::{
    hex::FromHex,
    network::TransactionResponse,
    primitives::{address, Address, TxHash},
    providers::{Provider, ProviderBuilder},
    rpc::types::{
        Block, BlockId, BlockTransactions, BlockTransactionsKind, Transaction,
    },
};
use eyre::eyre;
use log::debug;
use url::Url;

use crate::util::{get_block, get_transaction};

pub const ETH_SLOT_TIME: u64 = 15;
pub const L1_LOOKBACK: u64 = 10;
pub const TAIKO_SEQUENCER_ADDRESS: Address =
    address!("06a9Ab27c7e2255df1815E6CC0168d7755Feb19a");
pub const TAIKO_GENESIS_L1_BLOCK_NUMBER: BlockId = BlockId::number(19945276);
pub const TAIKO_GENESIS_L1_TX_HASH: &str =
    "0x75caf0c3cf9a13349fccb7b8a2ee134ba6a31e309a97f335115edeb7e7c9edd5";

#[derive(Clone, Debug)]
pub struct BlockInfo {
    pub l1_block: Block,
    pub l2_block: Block,
    pub proposal_tx: Transaction,
}

impl BlockInfo {
    pub fn new(
        l1_block: Block,
        l2_block: Block,
        proposal_tx: Transaction,
    ) -> Self {
        Self {
            l1_block,
            l2_block,
            proposal_tx,
        }
    }

    pub async fn from_l2_block(
        rpc: Url,
        l2_block: Block,
    ) -> eyre::Result<Option<Self>> {
        debug!(
            "Searching for L1 block for L2 block {}...",
            l2_block.header.number.unwrap()
        );
        let provider = ProviderBuilder::new().on_http(rpc.clone());

        if l2_block.header.number == Some(0) {
            return Ok(Some(Self::new(
                get_block(rpc.clone(), TAIKO_GENESIS_L1_BLOCK_NUMBER).await?,
                l2_block,
                get_transaction(
                    rpc,
                    TxHash::from_hex(TAIKO_GENESIS_L1_TX_HASH)?,
                )
                .await?,
            )));
        };

        let time_difference = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs()
            - l2_block.header.timestamp;

        let curr_l1_block_number = provider.get_block_number().await?;

        let start_block =
            curr_l1_block_number - time_difference / ETH_SLOT_TIME;

        for i in 0..L1_LOOKBACK {
            let curr_block = match provider
                .get_block(
                    (start_block - i).into(),
                    BlockTransactionsKind::Full,
                )
                .await?
            {
                Some(t) => t,
                None => return Err(eyre!("no such block")),
            };

            // TODO(jmcph4): search current block for (successful) Taiko proposal transaction
            if let BlockTransactions::Full(ref txs) = curr_block.transactions {
                if let Some(proposal_tx) = txs
                    .iter()
                    .filter(|tx| tx.to().is_some())
                    .find(|tx| tx.to().unwrap() == TAIKO_SEQUENCER_ADDRESS)
                {
                    return Ok(Some(Self::new(
                        curr_block.clone(),
                        l2_block,
                        proposal_tx.clone(),
                    )));
                }
            }
        }

        Ok(None)
    }
}
