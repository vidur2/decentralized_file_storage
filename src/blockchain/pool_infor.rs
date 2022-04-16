use datetime::Instant;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct PoolInfor {
    pub timestamp: i64,
    nodes_present: Vec<Vec<u8>>,
    pub tokens_in_period: f32
}

impl PoolInfor {
    pub fn new(tokens_in_period: f32) -> Self {
        let timestamp = Instant::now().seconds();
        return Self {
            timestamp,
            nodes_present: Self::get_nodes(),
            tokens_in_period,
        }
    }

    pub fn get_tokens_in_account(&self, acct: &Vec<u8>) -> f32 {
        let acct_amt = self.nodes_present.len();
        for acct_block in self.nodes_present.iter() {
            if acct_block == acct {
                return self.tokens_in_period/(acct_amt as f32)
            }
        }

        return 0.;
    }

    pub fn verify_pool_block(&self) -> bool{
        todo!()
    } 

    pub fn verify_block_time(&self) -> bool {
        if self.timestamp > Instant::now().seconds() {
            return false
        } else {
            return true
        }
    }

    fn get_nodes() -> Vec<Vec<u8>> {
        todo!()
    }
}