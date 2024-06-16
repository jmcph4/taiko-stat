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
    println!("Proposer: {}", block_info.l2_block.header.miner);

    Ok(())
}
