use openssl::sha::Sha256;
use serde::{Deserialize, Serialize};
use hex;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileEntry {
    pub index: usize,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub files: Vec<FileEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRequest {
    pub file: String,
}

impl FileRequest {
    pub fn deserialize(jdata: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(jdata)?)
    }
}

pub fn hash_password(password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finish();
    Ok(hex::encode(result))
}
