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
    #[clap(subcommand)]
    pub command: Commands,
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
