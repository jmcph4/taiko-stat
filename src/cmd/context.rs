use url::Url;

pub const DEFAULT_ETH_RPC_URL: &str = "https://eth.merkle.io";
pub const DEFAULT_TAIKO_RPC_URL: &str = "https://rpc.mainnet.taiko.xyz";

#[derive(Clone, Debug)]
pub struct CommandContext {
    pub eth_rpc: Url,
    pub taiko_rpc: Url,
}

impl CommandContext {
    pub fn new(eth_rpc: Option<Url>, taiko_rpc: Option<Url>) -> Self {
        Self {
            eth_rpc: eth_rpc.map_or(
                DEFAULT_ETH_RPC_URL
                    .parse()
                    .expect("invalid default Ethereum RPC URL"),
                |x| x,
            ),
            taiko_rpc: taiko_rpc.map_or(
                DEFAULT_TAIKO_RPC_URL
                    .parse()
                    .expect("invalid default Taiko RPC URL"),
                |x| x,
            ),
        }
    }
}
