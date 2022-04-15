use super::file_infor::{BlockInformation, FileType};
use datetime::Instant;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Struct to represent a single file on the blockchain
///
/// ## Fields
///
/// * `index`- Where the block is in the blockchain
/// * `previous_hash`- Sha256 representation of the data in the previous block
///     * Used for validation
///     * Contains the previous hash of the last block
/// * `data`- The actual file stored in the block
#[derive(Clone, Serialize, Deserialize, std::cmp::PartialEq, Debug)]
pub struct Block {
    pub index: u128,
    pub previous_hash: String,
    pub data: BlockInformation,
    pub hash: String,
}

impl Block {
    /// Constructor for single Block struct
    ///
    /// ## Arguments
    ///
    /// * `index`- Index of the block
    /// * `previous hash`- Previous hash of the block (used for verification)
    /// * `data`- Actual data stored in the block as a FileInformation struct
    pub fn new(index: u128, previous_hash: String, data: BlockInformation) -> Self {
        Self {
            index,
            previous_hash: previous_hash.clone(),
            data: data.clone(),
            hash: hash_block(index, previous_hash, data).unwrap(),
        }
    }

    /// Gets the genesis block (hardcoded information)
    pub fn genesis() -> Self {
        let timestamp = Instant::now().seconds() as i128;
        let data = BlockInformation::new(
            *b"tttttttttttttttttttttttttttttttt",
            Some(String::from("0.0.1")),
            Some(String::from("https://github.com/vidur2")),
            Some(b"test1"),
            Some(FileType::DataStore),
            b"initial signature".to_vec(),
            timestamp,
            0,
            Some(String::from("")),
        );

        return Self {
            index: 0,
            previous_hash: String::from("0"),
            data: data.clone().unwrap(),
            hash: hash_block(0, String::from("0"), data.unwrap()).unwrap(),
        };
    }
}

/// Function to hash block. Used for verification
pub fn hash_block(index: u128, previous_hash: String, data: BlockInformation) -> Option<String> {
    let mut hasher = Sha256::new();
    let data_as_str = serde_json::to_string(&data);

    match data_as_str {
        Ok(data_uw) => {
            let mut block_str = index.to_string();
            block_str.push_str(&previous_hash);
            block_str.push_str(&data_uw);
            hasher.update(block_str);
            return Some(format!("{:X}", hasher.finalize()));
        }
        Err(err) => {
            println!("{}", &err);
            return None;
        }
    }
}
