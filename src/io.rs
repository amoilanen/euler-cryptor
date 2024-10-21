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

pub fn stdin_stream() -> Result<BufReader<io::StdinLock<'static>>, anyhow::Error> {
    Ok(BufReader::new(io::stdin().lock()))
}

pub fn process_chunks_of<R: Read, F>(input: &mut BufReader<R>, chunk_size: usize, chunk_processor: F) -> Result<(), anyhow::Error>
where F: Fn(&Vec<u8>) -> Result<(), anyhow::Error> {
    let mut buffer = vec![0u8; chunk_size];
    let mut read_buffer_size = 0;
    let mut read_bytes_size = 1;
    while read_bytes_size != 0 {
        read_bytes_size = input.read(&mut buffer[read_buffer_size..])?;
        read_buffer_size = read_buffer_size + read_bytes_size;
        let has_finished_reading_chunk = (read_bytes_size == 0 && read_buffer_size > 0) || (read_buffer_size == chunk_size);
        if has_finished_reading_chunk {
            let read_bytes = buffer[0..read_buffer_size].to_vec();
            read_buffer_size = 0;
            chunk_processor(&read_bytes)?;
        }
    }
    Ok(())
}

pub fn write_to_stdout(bytes: &Vec<u8>) -> Result<(), anyhow::Error> {
    io::stdout().lock().write_all(bytes).map_err(Error::from)
}
