use anyhow::Result;
use base64::{decode, encode};
use std::fs;
use std::path::Path;

pub fn write_file(path: &str, data: &[u8]) -> Result<()> {
    fs::write(path, data)?;
    Ok(())
}

pub fn read_file(path: &str) -> Result<Vec<u8>> {
    Ok(fs::read(path)?)
}

pub fn encode_b64(data: &[u8]) -> String {
    encode(data)
}

pub fn decode_b64(data: &str) -> Result<Vec<u8>> {
    Ok(decode(data)?)
}

pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}
