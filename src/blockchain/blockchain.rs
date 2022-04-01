use serde::{Deserialize, Serialize};

use super::block::Block;
use super::file_infor::FileInformation;

#[derive(Deserialize, Serialize)]
pub struct Blockchain(pub Vec<Block>);


impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Vec::new();
        blockchain.push(Block::genesis());
        return Self(blockchain);
    }

    pub fn add_block(&mut self, file: FileInformation) -> bool {
        let next_index = self.0.len();
        let prev_block = &self.0[next_index - 1];
        let block = Block::new(next_index as u128, prev_block.hash_block().unwrap(), file);
        if self.check_block_validity(&block, &prev_block) {
            drop(prev_block);
            self.0.push(block);
            return true;
        } else {
            return false;
        }
    }

    pub fn find_block_by_uri(&self, uri: &str) -> Option<&Block> {
        let blockchain = &self.0;
        for block in blockchain.iter() {
            if block.data.linked_uri == uri {
                return Some(block);
            }
        }
        return None;
    }

    pub fn add_unverified_block(&mut self, new_block: Block) -> bool {
        if self.check_block_validity(&new_block, &self.0[self.0.len()]) {
            self.0.push(new_block);
            return true;
        } else {
            return false;
        }
    }

    // Checks whether a block is valid
    fn check_block_validity(&self, new_block: &Block, previous_block: &Block) -> bool {

        if new_block.index - 1 != previous_block.index || previous_block.hash_block().unwrap() != new_block.previous_hash {
            return false
        } else {
            return true
        }
    }

    // Checks whether an entered chain is valid
    fn check_chain_validity(&self, new_chain: &Vec<Block>) -> bool {

        // Return variable
        let mut is_valid = true;

        // Iterates through chain
        for i in 1..new_chain.len() - 1 {
            let current_block = &new_chain[i];
            let previous_block = &new_chain[i - 1];
            let block_validity = self.check_block_validity(current_block, previous_block);
            if !block_validity {
                is_valid = false;
            }
        }
    
        return is_valid
    }

    pub fn replace_chain(&mut self, replacement_chain: Vec<Block>) -> bool {
        if self.check_chain_validity(&replacement_chain) && self.0.len() < replacement_chain.len() {
            self.0 = replacement_chain;
            return true;
        }  else {
            return false;
        }
    }

}