use sha2::{ Sha256, Digest };
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FileInformation {
    pub data: String,
    pub linked_uri: String,
    creator: String,
    version: String
}

impl FileInformation {
    pub fn new(creator: String, version: String, linked_uri: String, data: String) -> Self {
        return Self {
            data,
            linked_uri,
            creator,
            version
        }
    }

    pub fn generate_b64_uri(self) -> String {
        todo!()
    }
}