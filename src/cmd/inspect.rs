use alloy::rpc::types::BlockId;
use eyre::eyre;
use log::info;

use crate::{
    cmd::CommandContext,
    types::BlockInfo,
    util::{display_block_id, get_block},
};

/// Inspects a Taiko block, reporting a summary to standard output
pub async fn block(
    ctx: CommandContext,
    identifier: BlockId,
) -> eyre::Result<()> {
    info!("Inspecting block {}...", display_block_id(identifier));
    let block_info: BlockInfo = match BlockInfo::from_l2_block(
        ctx.eth_rpc,
        get_block(ctx.taiko_rpc, identifier).await?,
    )
    .await?
    {
        Some(t) => t,
        None => return Err(eyre!("cannot find L1 block")),
    };

    println!("=== L1 Block ===");
    println!(
        "Number: {} Hash: {}",
        block_info.l1_block.header.number.unwrap(),
        block_info.l1_block.header.hash.unwrap(),
    );
    println!("Timestamp: {}", block_info.l1_block.header.timestamp);
    println!("State root: {}", block_info.l1_block.header.state_root);
    println!("Proposer: {}", block_info.l1_block.header.miner);

    println!();
    println!("=== L2 Block ===");
    println!(
        "Number: {} Hash: {}",
        block_info.l2_block.header.number.unwrap(),
        block_info.l2_block.header.hash.unwrap(),
    );
    println!("Timestamp: {}", block_info.l2_block.header.timestamp);
    println!("State root: {}", block_info.l2_block.header.state_root);
    println!(
        "Proposer: {} (in {})",
        block_info.l2_block.header.miner, block_info.proposal_tx.hash
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use alloy::{
        hex::FromHex,
        network::TransactionResponse,
        primitives::{BlockHash, TxHash},
    };

    use crate::{
        cmd::{DEFAULT_ETH_RPC_URL, DEFAULT_TAIKO_RPC_URL},
        types::TAIKO_GENESIS_L1_TX_HASH,
        util,
    };

    use super::*;

    const TAIKO_GENESIS_L1_BLOCK_HASH: &str =
        "0x8434095039c6b503ff60fe378a3829d00c82addc8ee06914cb33049cdf82621c";
    const TAIKO_GENESIS_L2_BLOCK_HASH: &str =
        "0x90bc60466882de9637e269e87abab53c9108cf9113188bc4f80bcfcb10e489b9";

    #[tokio::test]
    async fn test_from_l2_block_block0() {
        let res = BlockInfo::from_l2_block(
            DEFAULT_ETH_RPC_URL.parse().unwrap(),
            util::get_block(
                DEFAULT_TAIKO_RPC_URL.parse().unwrap(),
                BlockId::number(0),
            )
            .await
            .unwrap(),
        )
        .await;
        assert!(res.is_ok());
        let perhaps_info = res.unwrap();
        assert!(perhaps_info.is_some());
        let actual_info = perhaps_info.unwrap();

        /* avoid comparison of the entire blocks */
        assert_eq!(
            actual_info.l1_block.header.hash.unwrap(),
            BlockHash::from_hex(TAIKO_GENESIS_L1_BLOCK_HASH).unwrap()
        );
        assert_eq!(
            actual_info.l2_block.header.hash.unwrap(),
            BlockHash::from_hex(TAIKO_GENESIS_L2_BLOCK_HASH).unwrap()
        );
        assert_eq!(
            actual_info.proposal_tx.tx_hash(),
            TxHash::from_hex(TAIKO_GENESIS_L1_TX_HASH).unwrap()
        );
    }

    #[tokio::test]
    async fn test_from_l2_block_block1() {
        let res = BlockInfo::from_l2_block(
            DEFAULT_ETH_RPC_URL.parse().unwrap(),
            util::get_block(
                DEFAULT_TAIKO_RPC_URL.parse().unwrap(),
                BlockId::number(1),
            )
            .await
            .unwrap(),
        )
        .await;
        assert!(res.is_ok());
        let perhaps_info = res.unwrap();
        assert!(perhaps_info.is_some());
        let actual_info = perhaps_info.unwrap();

        /* avoid comparison of the entire blocks */
        assert_eq!(actual_info.l1_block.header.hash.unwrap(), BlockHash::from_hex("0x8434095039c6b503ff60fe378a3829d00c82addc8ee06914cb33049cdf82621c").unwrap());
        assert_eq!(actual_info.l2_block.header.hash.unwrap(), BlockHash::from_hex("0x2b58f652c63cadd274d48238175ba5fb2564d2813e96133b72b19d0be7cec895").unwrap());
        assert_eq!(actual_info.proposal_tx.tx_hash(), TxHash::from_hex("0x75caf0c3cf9a13349fccb7b8a2ee134ba6a31e309a97f335115edeb7e7c9edd5").unwrap());
    }
}
