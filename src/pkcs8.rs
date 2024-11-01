use crate::crypto::Key;
use anyhow::Result;
use yasna::{self, models::ObjectIdentifier};

// PKCS#8 https://datatracker.ietf.org/doc/html/rfc5208#appendix-A
#[derive(Default)]
pub(crate) struct PrivateKeyInfo {
    pub(crate) version: u32,
    pub(crate) private_key_algorithm: AlgorithmIdentifier,
    pub(crate) private_key: Vec<u8>
}

#[derive(Default)]
pub(crate) struct AlgorithmIdentifier {
    pub(crate) algorithm: Vec<u64>
}

const RSA_IDENTIFIER: [u64; 7] = [1, 2, 840, 113549, 1, 1, 1];

impl PrivateKeyInfo {

    pub(crate) fn wrap(key: &Key) -> PrivateKeyInfo {
        PrivateKeyInfo {
            version: 0,
            private_key_algorithm: AlgorithmIdentifier {
                algorithm: RSA_IDENTIFIER.to_vec()
            },
            private_key: key.as_bytes()
        }
    }

    pub(crate) fn deserialize(input: &[u8]) -> Result<PrivateKeyInfo, yasna::ASN1Error> {
        yasna::parse_der(input, |reader| {
            reader.read_sequence(|reader| {
                let version = reader.next().read_u32()?;
                let private_key_algorithm = reader.next().read_sequence(|reader| {
                    let algorithm = reader.next().read_oid()?.components().to_vec();
                    Ok(AlgorithmIdentifier { algorithm })
                })?;
                let private_key = reader.next().read_bytes()?;
                Ok(PrivateKeyInfo {
                    version,
                    private_key_algorithm,
                    private_key
                })
            })
        })
    }

    pub(crate) fn serialize(&self) -> Vec<u8> {
        yasna::construct_der(|writer| {
            writer.write_sequence(|writer| {
                writer.next().write_u32(self.version);
                writer.next().write_sequence(|writer| {
                    writer.next().write_oid(&ObjectIdentifier::from_slice(&self.private_key_algorithm.algorithm));
                });
                writer.next().write_bytes(&self.private_key);
            })
        })
    }
}