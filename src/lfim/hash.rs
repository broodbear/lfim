use std::fs;
use std::io::Result;
use sha2::{Digest, Sha256};

pub fn hash(path: &str) -> Result<String> {
    let mut hash = Sha256::new();

    let contents = fs::read(path).expect("couldn't read file");
    hash.update(contents);
    let result = hash.finalize();
    let hex_result = hex::encode(result);

    Ok(hex_result)
}
