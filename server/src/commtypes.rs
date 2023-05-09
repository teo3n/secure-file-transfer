use serde::{Deserialize, Serialize};

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
