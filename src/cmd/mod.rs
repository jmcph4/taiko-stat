pub(crate) mod context;
pub(crate) mod inspect;

use crate::types::*;

use alloy::rpc::types::BlockId;
use context::*;

pub fn print_taiko_contract_address() {
    println!("{}", TAIKO_SEQUENCER_ADDRESS);
}

pub fn print_taiko_genesis_l1_block_number() {
    let perhaps_number: u64 =
        if let BlockId::Number(number_or_tag) = TAIKO_GENESIS_L1_BLOCK_NUMBER {
            number_or_tag
                .as_number()
                .expect("illegal Taiko genesis block number")
        } else {
            panic!("illegal Taiko genesis block number")
        };
    println!("{:?}", perhaps_number);
}
