use alloy::rpc::types::BlockId;
use clap::{Parser, Subcommand};
use url::Url;

/// `taiko-stat` inspects Taiko blockchain objects
#[derive(Clone, Debug, Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub eth_rpc: Option<Url>,
    #[clap(short, long)]
    pub taiko_rpc: Option<Url>,
    /// Display the address of the Taiko sequencer contract
    #[clap(action, long)]
    pub print_taiko_contract_address: bool,
    /// Display the block number of the first mainnet block to contain a Taiko block
    #[clap(action, long)]
    pub print_taiko_genesis_l1_block_number: bool,
    /// Display the hash of the first mainnet transaction to propose a Taiko block
    #[clap(action, long)]
    pub print_taiko_genesis_l1_transaction_hash: bool,
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Commands {
    /// Inspect Taiko chain data
    Inspect {
        #[clap(subcommand)]
        command: InspectCommands,
    },
}

#[derive(Clone, Debug, Subcommand)]
pub enum InspectCommands {
    /// Inspect a Taiko (L2) block
    Block {
        /// The block hash or number to inspect
        identifier: BlockId,
    },
}
