use datetime::Instant;
use sha2::{ Sha256, Digest };
use serde::{Serialize, Deserialize};
use super::file_infor::{FileInformation, FileType};

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u128,
    pub previous_hash: String,
    timestamp: i128,
    pub data: FileInformation
}

impl Block {

    /// Constructor for single Block struct
    /// 
    /// # Arguments
    /// 
    /// * `index`- Index of the block
    /// * `previous hash`- Previous hash of the block (used for verification)
    /// * `data`- Actual data stored in the block as a FileInformation struct
    pub fn new(index: u128, previous_hash: String, data: FileInformation) -> Self {
        Self {
            index,
            previous_hash,
            timestamp: Instant::now().seconds() as i128,
            data
        }
    }

    /// Function to hash block. Used for verification
    pub fn hash_block(&self) -> Option<String> {
        let mut hasher = Sha256::new();
        let data_as_str = serde_json::to_string(&self);

        match data_as_str {
            Ok(data_uw) => {
                hasher.update(data_uw);
                return Some(format!("{:X}", hasher.finalize()))
            },
            Err(err) => {
                println!("{}", &err);
                return None
            }
        }
    }

    /// Gets the genesis block (hardcoded information)
    pub fn genesis() -> Self {
        let data = FileInformation::new(String::from("vidur2"), String::from("0.0.1"), String::from("https://github.com/vidur2"), b"test1", FileType::DataStore);

        return Self {
            index: 0,
            previous_hash: String::from(""),
            timestamp: Instant::now().seconds() as i128,
            data: data
        }
    }
}