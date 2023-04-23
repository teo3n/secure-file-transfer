use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub index: usize,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub files: Vec<FileEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRequest {
    pub file: String,
}


impl FileInfo {
    pub fn deserialize(jdata: &str) -> Self {
        serde_json::from_str::<FileInfo>(jdata).unwrap()
    }
}
