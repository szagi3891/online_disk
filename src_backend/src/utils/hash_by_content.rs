use crypto::sha1::Sha1;
use crypto::digest::Digest;

use utils::hex::{convert_from_hex};
use utils::hash::Hash;

pub fn hash_by_content(content: &[u8]) -> Hash {
    let mut hasher = Sha1::new();

    hasher.input(content);
    
    let hex = hasher.result_str();

    let hash_bin = convert_from_hex(hex.as_bytes());
    Hash::new(hash_bin)
}