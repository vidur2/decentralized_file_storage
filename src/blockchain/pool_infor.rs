use datetime::Instant;
use serde::{Deserialize, Serialize};

const GET_AMT_NODES: &str = "http://localhost:8080/get_amt_nodes";
const GET_NODES: &str = "http://localhost:8080/get_public_keys";

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct PoolInfor {
    pub timestamp: i64,
    nodes_present: Vec<Vec<u8>>,
    pub tokens_in_period: f32,
}

impl PoolInfor {
    pub fn new(tokens_in_period: f32) -> Self {
        let timestamp = Instant::now().seconds();
        return Self {
            timestamp,
            nodes_present: Self::get_nodes(),
            tokens_in_period,
        };
    }

    pub fn get_tokens_in_account(&self, acct: &Vec<u8>) -> f32 {
        let acct_amt = self.nodes_present.len();
        for acct_block in self.nodes_present.iter() {
            if acct_block == acct {
                return self.tokens_in_period / (acct_amt as f32);
            }
        }

        return 0.;
    }

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

    fn verify_block_time(&self) -> bool {
        if self.timestamp > Instant::now().seconds() {
            return false;
        } else {
            return true;
        }
    }

    pub fn get_amount_of_nodes() -> Option<usize> {
        match reqwest::blocking::get(GET_AMT_NODES) {
            Ok(amt_nodes) => match amt_nodes.text() {
                Ok(text) => return Some(text.trim().parse().unwrap()),
                Err(_) => None,
            },
            Err(_err) => return None,
        }
    }

    fn get_nodes() -> Vec<Vec<u8>> {
        let pk_as_str = reqwest::blocking::get(GET_NODES).unwrap().text().unwrap();
        let parsed: Vec<Vec<u8>> = serde_json::from_str(&pk_as_str).unwrap();

        return parsed;
    }
}
