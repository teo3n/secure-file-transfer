use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

use hex;
use openssl::sha::Sha256;
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};

use crate::consts::SALT_LEN;

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

pub fn gen_salt() -> String {
    let mut rng = OsRng;
    let mut salt = [0u8; SALT_LEN];
    rng.fill_bytes(&mut salt[..]);
    hex::encode(salt)
}

pub fn hash_password(password: &str, salt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let salt_hash = format!("{}{}", salt, password);

    let mut hasher = Sha256::new();
    hasher.update(salt_hash.as_bytes());
    let result = hasher.finish();
    Ok(hex::encode(result))
}

pub fn write_hash_to_file(
    path: &str,
    hash: &str,
    salt: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(path)?;
    let data = format!("{}+{}", salt, hash);
    file.write_all(data.as_bytes())?;

    Ok(())
}
pub fn read_salt_hash(path: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut salt = String::new();
    let mut hash = String::new();

    for line in reader.lines() {
        let line = line?;
        if let Some(pos) = line.find('+') {
            salt = line[..pos].to_string();
            hash = line[pos + 1..].to_string();
            break;
        }
    }

    Ok((salt, hash))
}
