use std::{fs::File, io::{Write, BufReader, BufRead}};

use openssl::sha::Sha256;
use rand::{rngs::OsRng, RngCore};

use crate::consts::SALT_LEN;

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