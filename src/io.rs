use std::io::{self, Read, BufReader};
use std::path::{ Path, PathBuf };
use std::fs::{ self, File, OpenOptions };
use std::io::{Write, BufRead};
use anyhow::{Result, Error};
use crate::crypto;

pub fn read_key_from(path: &Path) -> Result<crypto::Key, anyhow::Error> {
    let bytes = fs::read(path)?;
    crate::crypto::Key::deserialize(&bytes)
}

pub fn save_key_to(key: &crypto::Key, key_path: &Path) -> Result<(), anyhow::Error> {
    let mut public_key_file = File::create(key_path)?;
    public_key_file.write_all(&key.serialize())?;
    Ok(())
}

pub fn create_key_path(key_directory: &str, key_pair_name: &str, key_prefix: &str) -> PathBuf {
    let key_file_name = format!("{}_{}.pem", key_pair_name, key_prefix);
    Path::new(&key_directory).join(&key_file_name)
}

pub fn encrypt(reader: &mut Box<dyn BufRead>, writer: &mut Box<dyn Write>, key: &crypto::Key, chunk_size: usize) -> Result<(), anyhow::Error> {
    process_chunks_of(reader, writer, chunk_size, |chunk, writer| {
        let encrypted = crypto::encrypt_bytes(&chunk, &key);
        write_bytes(&encrypted, writer)
    })
}

pub fn decrypt(reader: &mut Box<dyn BufRead>, writer: &mut Box<dyn Write>, key: &crypto::Key, chunk_size: usize) -> Result<(), anyhow::Error> {
    process_chunks_of(reader, writer, chunk_size, |chunk, writer| {
        let decrypted = crypto::decrypt_bytes(&chunk, &key);
        write_bytes(&decrypted, writer)
    })
}

pub fn process_chunks_of<F>(input: &mut Box<dyn BufRead>, output: &mut Box<dyn Write>, chunk_size: usize, chunk_processor: F) -> Result<(), anyhow::Error>
where F: Fn(&Vec<u8>, &mut Box<dyn Write>) -> Result<(), anyhow::Error> {
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
            chunk_processor(&read_bytes, output)?;
        }
    }
    Ok(())
}

pub fn input_reader(input: &Option<String>) -> Result<Box<dyn BufRead>, anyhow::Error> {
    match input {
        Some(input_path) =>
            file_reader(&input_path),
        None =>
            stdin_reader()
    }
}

fn file_reader(input_path: &str) -> Result<Box<dyn BufRead>, anyhow::Error> {
    let file = OpenOptions::new().read(true).open(input_path)?;
    Ok(Box::new(BufReader::new(file)))
}

fn stdin_reader() -> Result<Box<dyn BufRead>, anyhow::Error> {
    Ok(Box::new(BufReader::new(io::stdin().lock())))
}

pub fn output_writer(output: &Option<String>) -> Result<Box<dyn Write>, anyhow::Error> {
    match output {
        Some(output_path) =>
            file_writer(&output_path),
        None =>
            stdout_writer()
    }
}

fn file_writer(output_path: &str) -> Result<Box<dyn Write>, anyhow::Error> {
    let file = OpenOptions::new().write(true).create(true).open(output_path)?;
    Ok(Box::new(file))
}

fn stdout_writer() -> Result<Box<dyn Write>, anyhow::Error> {
    Ok(Box::new(io::stdout().lock()))
}

pub fn write_bytes(bytes: &Vec<u8>, write: &mut Box<dyn Write>) -> Result<(), anyhow::Error> {
    write.write_all(bytes).map_err(Error::from)
}
