use std::fs;

use crate::compression;

pub mod blob;

pub fn get_path(object_hash: &str) -> String {
    let dir = &object_hash[0..2];
    let filename = &object_hash[2..];
    format!(".git/objects/{dir}/{filename}")
}

/// Reads and decompressed object contents
/// TODO: fix handling results
pub fn read_to_string(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path)?;
    let decoder = compression::Decoder::new(contents);
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed)?;
    Ok(s)
}
