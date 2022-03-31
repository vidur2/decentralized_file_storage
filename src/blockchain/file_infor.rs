use serde::{Serialize, Deserialize};
use tree_magic_fork;
use base64_url;

#[derive(Serialize, Deserialize)]
pub struct FileInformation {
    pub data: String,
    pub linked_uri: String,
    creator: String,
    version: String
}

impl FileInformation {
    pub fn new(creator: String, version: String, linked_uri: String, data: &[u8]) -> Self {
        let mime_type = tree_magic_fork::from_u8(data);
        let b64_data = base64_url::encode(data);
        let final_data_url = format!("data:{},{}", mime_type, b64_data);

        return Self {
            data: final_data_url,
            linked_uri,
            creator,
            version
        }
    }
}