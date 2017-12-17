use std::path::PathBuf;
use crypto::sha1::Sha1;
use crypto::digest::Digest;
use std::fmt::Write;

use blob::types::Fs;
use utils::hash::Hash;
use utils::hex::{convert_from_hex};

#[cfg(test)]
mod test;

pub struct BlobKeyValue<T> where T: Fs {
    data_path: PathBuf,
    fs: T,
}

impl<T> BlobKeyValue<T> where T : Fs {
    pub fn new(data_path: String, fs: T) -> BlobKeyValue<T> {
        BlobKeyValue {
            data_path: PathBuf::from(data_path),
            fs: fs
        }
    }

    pub fn set_blob(&self, content: Vec<u8>) {
        let mut hasher = Sha1::new();

        hasher.input(content.as_slice());
        
        let hex = hasher.result_str();
        
        let hash_bin = convert_from_hex(hex.as_bytes());
        
        let hash = Hash::new(hash_bin);

        let (prefix1, prefix2) = extract_prefix_hash(&hash);

        /*
        let hash_string = hash.to_hex();
        
        let mut file_path = self.data_path;
        file_path.push(hash_string.as_slice()[0, 2]);
        file_path.push(hash_string[2, 5]);
        */

        //zbuduj ścieżkę
        //w tą ścieżkę zapisz bufor

        //hash

        panic!("stop");
    }

    pub fn get_blob(&self, hash: &Hash) -> Vec<u8> {
        panic!("TODO");
    }

    pub fn get_fs(self) -> T {
        self.fs
    }
}

fn extract_prefix_hash(hash: &Hash) -> (String, String) {
    let prefix_byte0 = hash.get_prefix(0) as u16;
    let prefix_byte1 = hash.get_prefix(1) as u16;
    let prefix_byte2 = hash.get_prefix(2) as u16;

    let prefix1: u16 = prefix_byte0 << 4 | ((prefix_byte1 & 0xf0) >> 4);
    let prefix2: u16 = ((prefix_byte1 & 0x0f) << 8) | prefix_byte2;

    (to_hex_u16(prefix1), to_hex_u16(prefix2))
}

#[test]
fn test_for_extract_prefix_hash() {
    let hash = Hash::new([12, 33, 44, 120, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let (pref1, pref2) = extract_prefix_hash(&hash);
    assert_eq!(hash.to_hex(), "0c212c781e000000000000000000000000000000");
    assert_eq!(pref1, "0c2");
    assert_eq!(pref2, "12c");
}

#[test]
fn test_for_extract_prefix_hash2() {
    let hash = Hash::new([99, 88, 250, 120, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let (pref1, pref2) = extract_prefix_hash(&hash);
    assert_eq!(hash.to_hex(), "6358fa781e000000000000000000000000000000");
    assert_eq!(pref1, "635");
    assert_eq!(pref2, "8fa");
}

fn to_hex_u16(input: u16) -> String {
    let mut out = String::new();
    write!(&mut out, "{:03x}", input).unwrap();
    out
}

