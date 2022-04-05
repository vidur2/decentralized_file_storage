use serde::{Serialize, Deserialize};
use tree_magic_fork;
use base64_url;

/// Enum to represent how to serve the file
#[derive(Serialize, Clone, Deserialize, std::cmp::PartialEq)]
pub enum FileType {
    Frontend,
    DataStore
}

/// A file is represented here
#[derive(Serialize, Clone, Deserialize, std::cmp::PartialEq)]
pub struct FileInformation {

    /// Public field of data is a base64 encoded url
    pub data: String,

    /// Public field of linked uri is the domain that is hosting the data
    pub linked_uri: String,

    /// Creator of file for uniqueness purposes in hashing
    creator: String,

    /// Same purpose as above field
    version: String,

    /// Field defines how file should be served
    file_type: FileType
}

impl FileInformation {

    /// Constructor for FileInformation Object
    /// 
    /// # Arguments
    /// 
    /// * `creator`- An owned string of the user who created the file
    /// * `version`- An owned string representation of the file
    /// * `linked_uri`- An owned string representing the domain that is attatched to the file
    /// * `data`- a u8 representation of the bytes of the file
    pub fn new(creator: String, version: String, linked_uri: String, data: &[u8], file_type: FileType) -> Self {
        let mime_type = tree_magic_fork::from_u8(data);
        let b64_data = base64_url::encode(data);
        let final_data_url = format!("data:{},{}", mime_type, b64_data);

        return Self {
            data: final_data_url,
            linked_uri,
            creator,
            version,
            file_type
        }
    }
}