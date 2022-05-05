use serde::Serialize;

#[derive(Serialize)]
pub struct RemoveHostOptions {
    timestamp: String,
    signature: String,
    public_key: String
}

impl RemoveHostOptions {
    pub fn new(timestamp: String, signature: String, public_key: String) -> Self {
        return Self {
            timestamp,
            signature,
            public_key
        }
    }
}