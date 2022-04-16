use base64_url;
use serde::{Deserialize, Serialize};
use tree_magic_fork;

/// Enum to represent how to serve the file
#[derive(Serialize, Clone, Deserialize, std::cmp::PartialEq, Debug)]
pub enum FileType {
    Frontend,
    DataStore,
}

/// A file is represented here
#[derive(Serialize, Clone, Deserialize, std::cmp::PartialEq, Debug)]
pub struct BlockInformation {
    /// Public field of data is a base64 encoded url
    pub data: Option<String>,

    /// Public field of linked uri is the domain that is hosting the data
    pub linked_uri: String,

    /// Creator of file for uniqueness purposes in hashing
    pub(crate) creator: Vec<u8>,

    /// Same purpose as above field
    pub(crate) version: String,

    /// Field defines how file should be served
    pub(crate) file_type: FileType,

    /// Signature for verification
    pub signature: Vec<u8>,

    /// Timestamp of file (used as nonce)
    pub timestamp: i128,

    /// Tokens transferred
    pub tokens_transferred: f32,

    /// To acct id:
    pub to_acct_id: String,
}

impl BlockInformation {
    /// Constructor for FileInformation Object
    ///
    /// ## Arguments
    ///
    /// * `creator`- An owned string of the user who created the file
    /// * `version`- An owned string representation of the file
    /// * `linked_uri`- An owned string representing the domain that is attatched to the file
    /// * `data`- a u8 representation of the bytes of the file
    pub fn new(
        creator: [u8; 32],
        version: Option<String>,
        linked_uri: Option<String>,
        data: Option<&[u8]>,
        file_type: Option<FileType>,
        signature: Vec<u8>,
        timestamp: i128,
        tokens_transferred: f32,
        to_acct_id: Option<String>,
    ) -> Option<Self> {
        match data {
            Some(file_inf) => {
                let mime_type = tree_magic_fork::from_u8(file_inf);
                let b64_data = base64_url::encode(file_inf);
                let final_data_url = format!("data:{},{}", mime_type, b64_data);
                if tokens_transferred == file_inf.len() as f32 / 1_000_000_000. {
                    return Some(Self {
                        data: Some(final_data_url),
                        linked_uri: linked_uri.unwrap(),
                        creator: creator.to_vec(),
                        version: version.unwrap(),
                        file_type: file_type.unwrap(),
                        signature,
                        timestamp,
                        tokens_transferred,
                        to_acct_id: String::from("network"),
                    });
                } else {
                    return None;
                }
            }
            None => {
                return Some(Self {
                    data: None,
                    linked_uri: String::from(""),
                    creator: creator.to_vec(),
                    version: String::from(""),
                    file_type: FileType::DataStore,
                    signature,
                    timestamp,
                    tokens_transferred,
                    to_acct_id: to_acct_id.unwrap(),
                })
            }
        }
    }

    pub fn verify_signature(&self) -> bool {
        let information = self.linked_uri.clone()
            + &self.timestamp.to_string()
            + &self.tokens_transferred.to_string();
        let signature = ed25519_dalek::Signature::from_bytes(self.signature.as_slice());
        let public_key = ed25519_dalek::PublicKey::from_bytes(&self.creator);
        match public_key
            .unwrap()
            .verify_strict(information.as_bytes(), &signature.unwrap())
        {
            Ok(_val) => return true,
            Err(_err) => return false,
        }
    }
}
