use datetime::Instant;
use serde::{Deserialize, Serialize};

const GET_AMT_NODES: &str = "http://localhost:8080/get_amt_nodes";


/// Struct representing data that sends tokens from the network to the node
/// ## Fields
/// * `timestamp`: Representing the time the block was added
/// * `nodes_present`: The nodes that got the currency
/// * `tokens_in_period`: Amount of tokesn thtat the network generated in period 

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct PoolInfor {
    pub timestamp: i64,
    nodes_present: Vec<Vec<u8>>,
    pub tokens_in_period: f32,
}

impl PoolInfor {

    /// Constructor for PoolInfor
    /// ## Arguments
    /// * `tokens_in_period`: The amount of tokens generated in the period
    pub fn new(tokens_in_period: f32) -> Self {
        let timestamp = Instant::now().seconds();
        return Self {
            timestamp,
            nodes_present: Self::get_nodes(),
            tokens_in_period,
        };
    }

    /// Gets the amount of tokens a node is owed over a mining period
    /// 
    /// ## Arguments
    /// * `acct`: The account being found
    /// 
    /// ## Returns
    /// * `amount_owed`: f32, the amount the account is owed
    pub fn get_tokens_owed(&self, acct: &Vec<u8>) -> f32 {
        let acct_amt = self.nodes_present.len();
        for acct_block in self.nodes_present.iter() {
            if acct_block == acct {
                return self.tokens_in_period / (acct_amt as f32);
            }
        }

        return 0.;
    }

    /// Verification for a block
    pub fn verify_pool_block(&self) -> bool {
        match Self::get_amount_of_nodes() {
            Some(node_amt) => {
                if node_amt == self.nodes_present.len() && self.verify_block_time() {
                    return true;
                } else {
                    return false;
                }
            }
            None => return false,
        }
    }


    /// Verifies the timestamp of the block
    fn verify_block_time(&self) -> bool {
        if self.timestamp > Instant::now().seconds() {
            return false;
        } else {
            return true;
        }
    }

    /// Gets the amount of nodes in a period
    fn get_amount_of_nodes() -> Option<usize> {
        match reqwest::blocking::get(GET_AMT_NODES) {
            Ok(amt_nodes) => match amt_nodes.text() {
                Ok(text) => return Some(text.trim().parse().unwrap()),
                Err(_) => None,
            },
            Err(_err) => return None,
        }
    }

    fn get_nodes() -> Vec<Vec<u8>> {
        todo!()
    }
}
