use sha3::Keccak256;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Outputs {
    pub phishing_address: String,
    pub phished_address: String,
    pub transaction_hash: String,
}
