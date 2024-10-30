use crate::crypto;
use anyhow::{anyhow, Result};
use base64::prelude::*;

const LINE_SIZE: usize = 64;

pub(crate) fn serialize(key_data: &[u8], key_type: &crypto::KeyType) -> Vec<u8> {
    let encoded = BASE64_STANDARD.encode(key_data);
    let encoded_lines = encoded.chars().collect::<Vec<_>>().chunks(LINE_SIZE).map(|chunk| {
        chunk.iter().collect::<String>()
    }).collect::<Vec<_>>().join("\n");
    let mut result: Vec<u8> = Vec::new();
    let key_type = if key_type == &crypto::KeyType::Private {
        "PRIVATE"
    } else {
        "PUBLIC"
    };
    result.extend(format!("-----BEGIN {} KEY-----\n", key_type).as_bytes());
    result.extend(encoded_lines.as_bytes());
    result.extend(format!("\n-----END {} KEY-----\n", key_type).as_bytes());
    result
}

pub(crate) fn deserialize(input: &[u8]) -> Result<(Vec<u8>, crypto::KeyType), anyhow::Error> {
    let input_str = String::from_utf8(input.to_vec())?;
    let lines: Vec<&str> = input_str.split_terminator('\n').collect();
    let mut encoded_key: Vec<u8> = Vec::new();
    for s in lines[1..lines.len() - 1].iter() {
        encoded_key.extend(s.as_bytes());
    }
    let header = lines.get(0).ok_or(anyhow!("Could not find header"))?;
    let key_type = if header.contains("PRIVATE") {
        crypto::KeyType::Private
    } else {
        crypto::KeyType::Public
    };
    let decoded = BASE64_STANDARD.decode(encoded_key)?;
    Ok((decoded, key_type))
}