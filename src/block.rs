use chrono::Utc;
use sha2::{Digest, Sha256};

pub struct Block {
    pub id: u64,
    
    // Header
    pub timestamp: i64,
    pub previous_hash: String,
    pub nonce: u64,

    // Body
    pub data: String,
    pub hash: String,
}

pub fn calculate_hash(id: u64, timestamp: i64, previous_hash: &str, data: &str, nonce: u64) -> String {
    // Calculate the hash of a block using the SHA256 algorithm
    let mut hasher = Sha256::new();
    hasher.update(id.to_string());
    hasher.update(timestamp.to_string());
    hasher.update(previous_hash);
    hasher.update(data);
    hasher.update(nonce.to_string());
    format!("{:x}", hasher.finalize())
}

impl Block {
    pub fn new(id: u64, previous_hash: String, data: String) -> Self {
        let now = Utc::now();
        let timestamp = now.timestamp();
        let (nonce, hash) = mine_block(id, now.timestamp(), &previous_hash, &data);
        let hash = calculate_hash(id, timestamp, &previous_hash, &data, nonce);
        Self {
            id,
            timestamp,
            previous_hash,
            nonce,
            data,
            hash,
        }
    }
}