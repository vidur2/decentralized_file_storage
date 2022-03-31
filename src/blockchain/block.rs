use datetime::Instant;
use sha2::{ Sha256, Digest };
use serde::Serialize;
use super::file_infor::FileInformation;

#[derive(Clone, Serialize)]
pub struct Block {
    pub index: u128,
    pub previous_hash: String,
    pub linked_uri: String,
    timestamp: i128,
    pub data: String
}

impl Block {
    pub fn new(index: u128, previous_hash: String, data: FileInformation) -> Option<Self> {
        let mut hasher = Sha256::new();

        match serde_json::to_string(&data) {
            Ok(data_as_string) => {
                hasher.update(data_as_string);
                let data_enc = hasher.finalize();   

                Some(Self {
                    index,
                    linked_uri: data.linked_uri,
                    previous_hash,
                    timestamp: Instant::now().seconds() as i128,
                    data: format!("{:X}", data_enc)
                })
            },

            Err(err) => {
                println!("{}", &err);
                return None
            }
        }
    }

    pub fn find_block_by_uri(uri: String) -> String {
        todo!()
    }

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

    pub fn genesis() -> Self {
        let mut hasher = Sha256::new();
        let data = FileInformation::new(String::from("vidur2"), String::from("0.0.1"), String::from("https://github.com/vidur2"), String::from("Test1"));

        let data_as_string = serde_json::to_string(&data);
        drop(data);

        hasher.update(data_as_string.unwrap());
        let output = hasher.finalize();

        return Self {
            index: 0,
            linked_uri: String::from("https://github.com/vidur2"),
            previous_hash: String::from(""),
            timestamp: Instant::now().seconds() as i128,
            data: format!("{:X}", output)
        }
    }
}