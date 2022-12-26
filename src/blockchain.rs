use crate::block::{Block, calculate_hash};
use log::error;

const DIFFICULTY: u32 = 2;

pub struct Blockchain {
    pub chain: Vec<Block>,
}

fn hash_to_binary(hash: &str) -> Vec<u8> {
    let mut result = Vec::new();
    for i in 0..hash.len() / 2 {
        let byte = u8::from_str_radix(&hash[i * 2..i * 2 + 2], 16).unwrap();
        result.push(byte);
    }
    result
}

impl Blockchain {
    fn new() -> Self {
        Self {
            chain: Vec::new(),
        }
    }

    fn try_add_block(&mut self, block: Block) {
        let latest_block = self.chain.last().expect("there is at least one block");
        if self.is_block_valid(&block, latest_block) {
            self.chain.push(block);
        } else {
            error!("could not add block - invalid");
        }
    }

    fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
        if block.hash != block.previous_hash {
            return false;
        }
        if !hash_to_binary(&block.hash).starts_with(&vec![0; DIFFICULTY as usize]) {
            return false;
        }
        if block.id != previous_block.id + 1 {
            return false;
        }
        if calculate_hash(
            block.id,
            block.timestamp,
            &block.previous_hash,
            &block.data,
            block.nonce,
        ) != block.hash {
            return false;
        }
        true
    }

    fn is_chain_valid(&self, chain: &[Block]) -> bool {
        if chain.len() == 0 {
            return false;
        }
        if chain[0].hash != "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string() {
            return false;
        }
        for i in 1..chain.len() {
            let block = &chain[i];
            let previous_block = &chain[i - 1];
            if !self.is_block_valid(block, previous_block) {
                return false;
            }
        }
        true
    }
}