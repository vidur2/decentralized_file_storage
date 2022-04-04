use std::sync::{Mutex, Arc};

use serde::{Deserialize, Serialize};

use super::block::Block;
use super::file_infor::FileInformation;

pub type SharedChain =  Arc<Mutex<Blockchain>>;

#[derive(Deserialize, Serialize)]
pub struct Blockchain(pub Vec<Block>);


impl Blockchain {

    /// Constructor for blockchain
    pub fn new() -> Self {
        let mut blockchain = Vec::new();
        blockchain.push(Block::genesis());
        return Self(blockchain);
    }

    /// Adds block to blockchain
    /// 
    /// # Arguments
    /// 
    /// * `file`: A struct of type FileInformation that contains the data in the file being addded
    /// 
    /// # Returns
    /// A boolean indicating whether adding the block was succesful
    /// 
    pub fn add_block(&mut self, file: FileInformation) -> bool {
        let next_index = self.0.len();
        let prev_block = &self.0[next_index - 1];
        let block = Block::new(next_index as u128, prev_block.hash_block().unwrap(), file.clone());
        if self.check_block_validity(&block, &prev_block) && !self.check_if_exists(file) {
            self.0.push(block);
            return true;
        } else {
            return false;
        }
    }

    /// Gets the latest iteration of a Block containing the FileInformation for a specified uri
    /// 
    /// # Arguments
    /// * `uri`: The uri/url that is used to search for the FilInformation
    /// 
    /// # Returns
    /// An optional immutable reference to a Block
    /// * If the uri is in the block chain, the option will be non-none
    pub fn find_block_by_uri(&self, uri: &str) -> Option<&Block> {
        let blockchain = &self.0;
        for block in blockchain.iter().rev() {
            if block.data.linked_uri == uri {
                return Some(block);
            }
        }
        return None;
    }

    /// Method used to add verify and add a block pushed over a websocket
    /// 
    /// # Arguments
    /// * `new_block`: The block being added
    /// 
    /// # Returns
    /// Boolean status on whether block is valid
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

    fn check_if_exists(&self, file: FileInformation) -> bool {
        let serialized_file = serde_json::to_string(&file);

        match serialized_file {
            Ok(uw_file) => {
                let serialized_bc = serde_json::to_string(&self);
                match serialized_bc {
                    Ok(serialied_bc_uw) => {
                        if serialied_bc_uw.contains(&uw_file) {
                            return true;
                        } else {
                            return false;
                        }
                    },
                    Err(_) => {
                        return true;
                    },
                }
            },
            Err(_) => {
                return true;
            },
        }

    }

}