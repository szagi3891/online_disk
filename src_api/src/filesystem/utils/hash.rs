use std::path::PathBuf;

use filesystem::utils::hex::to_hex;
use serde::{Serialize, Serializer};

#[derive(PartialEq, Hash, Clone, Deserialize, Debug)]
pub struct Hash {
    hash: [u8; 20],
}

impl Eq for Hash {}

impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let hex = self.to_hex();
        serializer.serialize_str(&hex)
    }
}

impl Hash {
    pub fn new(hash: [u8; 20]) -> Hash {
        
        Hash {
            hash: hash
        }
    }

    pub fn add_to_path(&self, path: &mut PathBuf) {        
        let slice = &self.hash[..];
                                                //TODO - użyć lepszej metody do konwersji na hex
        let out = to_hex(slice);
        assert!(out.len() == 40);
        path.push(out);
    }

    pub fn get_prefix(&self, pos: u8) -> u8 {
        self.hash[pos as usize]
    }

                                                //TODO - dobrze byłoby to zrobić bez tylu alokacji przy serializowaniu danych
    pub fn to_hex(&self) -> String {
        to_hex(&self.hash)
    }

                                                //TODO - zmienić potem znowu na seiralize
    pub fn serialize(&self, out: &mut Vec<u8>) {
        for item in &self.hash {
            out.push(*item);
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.serialize(&mut out);
        out
    }

    pub fn from_string_bytes(bytes: &[u8]) -> Hash {
        assert_eq!(bytes.len(), 40);

        let mut out = [
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ];

        use std::str;

        for (index, item) in bytes.chunks(2).enumerate() {
            let chunk_str = str::from_utf8(item).unwrap();
            out[index] = u8::from_str_radix(chunk_str, 16).unwrap();
        }

        Hash::new(out)
    }

    pub fn from_string(data: &str) -> Hash {
        let bytes = data.as_bytes();
        Hash::from_string_bytes(bytes)
    }
}
