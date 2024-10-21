use std::io::{self, Read, BufReader};
use std::path::{ Path, PathBuf };
use std::fs::{ self, File };
use std::io::Write;
use anyhow::{Result, Error};
use crate::crypto;

pub fn read_key_from(path: &Path) -> Result<crypto::Key, anyhow::Error> {
    let bytes = fs::read(path)?;
    crate::crypto::Key::from_bytes(&bytes)
}

pub fn save_key_to(key: &crypto::Key, key_path: &Path) -> Result<(), anyhow::Error> {
    let mut public_key_file = File::create(key_path)?;
    public_key_file.write_all(&key.as_bytes())?;
    Ok(())
}

pub fn create_key_path(key_directory: &str, key_pair_name: &str, key_prefix: &str) -> PathBuf {
    let public_key_file_name = format!("{}_{}.elr", key_pair_name, key_prefix);
    Path::new(&key_directory).join(&public_key_file_name)
}

pub fn read_from_stdin() -> Result<Vec<u8>, anyhow::Error> {
    let mut buffer: Vec<u8> = Vec::new();
    io::stdin().lock().read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn stdin_stream() -> Result<BufReader<io::StdinLock<'static>>, anyhow::Error> {
    Ok(BufReader::new(io::stdin().lock()))
}

pub fn write_to_stdout(bytes: &Vec<u8>) -> Result<(), anyhow::Error> {
    io::stdout().lock().write_all(bytes).map_err(Error::from)
}

