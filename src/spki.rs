use crate::crypto::{Key, KeyType};
use crate::crypto;
use anyhow::{Result, Error};
use yasna::{self, models::ObjectIdentifier};

const RSA_IDENTIFIER: [u64; 7] = [1, 2, 840, 113549, 1, 1, 1];

//Subject Public Key Info from https://www.itu.int/rec/T-REC-X.509
pub(crate) struct SubjectPublicKeyInfo {
    pub(crate) public_key_algorithm: AlgorithmIdentifier,
    pub(crate) public_key: crypto::Key
}

pub(crate) struct AlgorithmIdentifier {
    pub(crate) algorithm: Vec<u64>
}

impl SubjectPublicKeyInfo {

    pub(crate) fn wrap(key: &Key) -> SubjectPublicKeyInfo {
        SubjectPublicKeyInfo {
            public_key_algorithm: AlgorithmIdentifier {
                algorithm: RSA_IDENTIFIER.to_vec()
            },
            public_key: key.clone()
        }
    }

    pub(crate) fn deserialize(input: &[u8]) -> Result<SubjectPublicKeyInfo, yasna::ASN1Error> {
        yasna::parse_der(input, |reader| {
            reader.read_sequence(|reader| {
                let public_key_algorithm = reader.next().read_sequence(|reader| {
                    let algorithm = reader.next().read_oid()?.components().to_vec();
                    Ok(AlgorithmIdentifier { algorithm })
                })?;
                let public_key = reader.next().read_bitvec_bytes()?.0;
                Ok(SubjectPublicKeyInfo {
                    public_key_algorithm,
                    public_key: Key::from_bytes(&public_key, KeyType::Public)?
                })
            })
        })
    }

    pub(crate) fn serialize(&self) -> Vec<u8> {
        yasna::construct_der(|writer| {
            writer.write_sequence(|writer| {
                writer.next().write_sequence(|writer| {
                    writer.next().write_oid(&ObjectIdentifier::from_slice(&self.public_key_algorithm.algorithm));
                });
                let public_key_bytes = self.public_key.as_bytes();
                writer.next().write_bitvec_bytes(&public_key_bytes, public_key_bytes.len() * 8);
            })
        })
    }
}