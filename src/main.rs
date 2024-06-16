use clap::Parser;

use crate::{
    cli::{Cli, Commands, InspectCommands},
    cmd::context::CommandContext,
};

mod cli;
mod cmd;
mod types;
mod util;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    pretty_env_logger::init_timed();

    let opts: Cli = Cli::parse();

    let ctx: CommandContext = CommandContext::new(opts.eth_rpc, opts.taiko_rpc);

    match opts.command {
        Commands::Inspect { command } => match command {
            InspectCommands::Block { identifier } => {
                cmd::inspect::block(ctx, identifier)
            }
        },
    }
    .await
}
