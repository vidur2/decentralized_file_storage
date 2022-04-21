use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use super::block::{Block, DataTypes};
use super::block_infor::BlockInformation;
use super::pool_infor::PoolInfor;

pub type SharedChain = Arc<Mutex<Blockchain>>;

#[derive(Deserialize, Serialize, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    /// Constructor for blockchain
    /// * Takes no arguments
    /// * Default block is always the original genesis block
    pub fn new() -> Self {
        let mut blockchain = Vec::new();
        blockchain.push(Block::genesis());
        return Self { chain: blockchain };
    }

    /// Adds block to blockchain
    ///
    /// ## Arguments
    ///
    /// * `file`: A struct of type FileInformation that contains the data in the file being addded
    ///
    /// ## Returns
    /// A boolean indicating whether adding the block was succesful
    ///
    pub fn add_block(&mut self, file: BlockInformation) -> bool {
        let next_index = self.chain.len();
        let prev_block = &self.chain[next_index - 1];
        let block = Block::new(
            next_index as u128,
            prev_block.hash.clone(),
            Some(DataTypes::Transaction(file.clone())),
        );
        let valid = self.check_block_validity(&block, &prev_block) && !self.check_if_exists(file);
        if valid {
            self.chain.push(block);
            return true;
        } else {
            return false;
        }
    }

    /// Gets the latest iteration of a Block containing the FileInformation for a specified uri
    ///
    /// ## Arguments
    /// * `uri`: The uri/url that is used to search for the FilInformation
    ///
    /// ## Returns
    /// An optional immutable reference to a Block
    /// * If the uri is in the block chain, the option will be non-none
    pub fn find_block_by_uri(&self, uri: &str) -> Option<&Block> {
        let blockchain = &self.chain;
        for block in blockchain.iter().rev() {
            if let Some(DataTypes::Transaction(txn)) = &block.data {
                if txn.linked_uri == uri {
                    return Some(block);
                }
            }
        }
        return None;
    }

    /// Method used to add verify and add a block pushed over a websocket
    ///
    /// ## Arguments
    /// * `new_block`: The block being added
    ///
    /// ## Returns
    /// Boolean status on whether block is valid
    pub fn add_unverified_block(&mut self, new_block: Block) -> bool {
        if self.check_block_validity(&new_block, &self.chain.last().unwrap()) {
            self.chain.push(new_block);
            return true;
        } else {
            return false;
        }
    }

    /// Checks whether a block is valid
    /// * Used to verify adding of files over websocket
    /// * Called in add_unverified_block method
    fn check_block_validity(&self, new_block: &Block, previous_block: &Block) -> bool {
        if let Some(DataTypes::Transaction(txn)) = &new_block.data {
            // Add size check here
            let valid = txn.verify_signature()
                && self.check_block_validity_balance(new_block)
                && self.check_to_file_id(txn);

            if new_block.index - 1 != previous_block.index
                || previous_block.hash != new_block.previous_hash
                || !valid
            {
                return false;
            } else {
                return true;
            }
        } else {
            return true;
        }
    }

    fn check_to_file_id(&self, txn: &BlockInformation) -> bool {
        if let Some(_) = txn.data {
            if txn.to_acct_id == *b"network".to_vec() {
                return true;
            } else {
                return false;
            }
        } else {
            return true;
        }
    }

    /// Checks whether an entered chain is valid
    /// * Used to verify blockchain recieved over websocket
    fn check_chain_validity(&self, new_chain: &Vec<Block>) -> bool {
        // Iterates through chain
        for i in 1..new_chain.len() - 1 {
            let current_block = &new_chain[i];
            let previous_block = &new_chain[i - 1];
            let block_validity = self.check_block_validity(current_block, previous_block);

            if let Some(DataTypes::Transaction(txn)) = &current_block.data {
                if !block_validity || txn.timestamp > datetime::Instant::now().seconds() as i128 {
                    return false;
                }
            } else if i > self.chain.len() - 1 && current_block != &self.chain[i] {
                return false
            } else if let Some(DataTypes::Withdrawal(pool_block)) = &current_block.data && i > self.chain.len() - 1 {
                if Self::calc_pool_amt(new_chain) == pool_block.tokens_in_period && pool_block.verify_pool_block() {
                    return true
                } else {
                    return false
                }
            }
        }

        return true;
    }

    pub fn get_amt_in_wallet(&self, acct: &Vec<u8>) -> f32 {
        let mut balance: f32 = 0.;
        for block in self.chain.iter() {
            if let Some(DataTypes::Transaction(txn)) = &block.data {
                if &txn.creator == acct {
                    balance -= txn.tokens_transferred
                } else if &txn.creator == acct {
                    balance += txn.tokens_transferred
                }
            } else if let Some(DataTypes::Withdrawal(pool_block)) = &block.data {
                balance += pool_block.get_tokens_owed(acct);
            }
        }

        return balance;
    }

    fn check_block_validity_balance(&self, block: &Block) -> bool {
        if let Some(DataTypes::Transaction(txn)) = &block.data {
            let creator = &txn.creator;
            let amt = self.get_amt_in_wallet(creator);
            if amt - txn.tokens_transferred as f32 >= 0. {
                return true;
            } else {
                return false;
            }
        } else {
            // Add up balance here
            return false;
        }
    }

    /// Function used to replace a chain recieved over websocket
    pub fn replace_chain(&mut self, replacement_chain: Vec<Block>) -> bool {
        if self.check_chain_validity(&replacement_chain) {
            self.chain = replacement_chain;
            return true;
        } else {
            return false;
        }
    }

    fn check_if_exists(&self, file: BlockInformation) -> bool {
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
                    }
                    Err(_) => {
                        return true;
                    }
                }
            }
            Err(_) => {
                return true;
            }
        }
    }

    pub fn withdraw(&mut self) -> bool {
        let pool_amt = Self::calc_pool_amt(&self.chain);
        let data = DataTypes::Withdrawal(PoolInfor::new(pool_amt));
        let block = Block::new(
            self.chain.len() as u128,
            self.chain.last().unwrap().hash.clone(),
            Some(data),
        );
        self.add_unverified_block(block)
    }

    fn calc_pool_amt(chain: &Vec<Block>) -> f32 {
        let mut pool_sum = 0.;
        for block in chain.iter().rev() {
            if let Some(data) = block.data.clone() {
                match data {
                    DataTypes::Transaction(txn) => {
                        if txn.to_acct_id == b"network".to_vec() {
                            pool_sum += txn.tokens_transferred
                        }
                    }
                    DataTypes::Withdrawal(_) => break,
                }
            }
        }
        return pool_sum;
    }
}
