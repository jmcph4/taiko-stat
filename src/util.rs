use alloy::{
    primitives::TxHash,
    providers::{Provider, ProviderBuilder},
    rpc::types::{Block, BlockId, BlockTransactionsKind, Transaction},
};
use eyre::eyre;
use log::debug;
use url::Url;

pub fn display_block_id(ident: BlockId) -> String {
    match ident {
        BlockId::Number(x) => format!("{x}"),
        BlockId::Hash(x) => format!("{}", x.block_hash),
    }
}

pub async fn get_block(rpc: Url, ident: BlockId) -> eyre::Result<Block> {
    debug!(
        "Retrieving block {} from {}...",
        display_block_id(ident),
        rpc
    );

    if let Some(block) = ProviderBuilder::new()
        .on_http(rpc)
        .get_block(ident, BlockTransactionsKind::Full)
        .await?
    {
        debug!("Block other fields: {:?}", block.other);
        Ok(block)
    } else {
        Err(eyre!("no block received"))
    }
}

pub async fn get_transaction(
    rpc: Url,
    hash: TxHash,
) -> eyre::Result<Transaction> {
    debug!("Retrieving tx {} from {}...", hash, rpc);

    if let Some(tx) = ProviderBuilder::new()
        .on_http(rpc)
        .get_transaction_by_hash(hash)
        .await?
    {
        Ok(tx)
    } else {
        Err(eyre!("no tx received"))
    }
}
