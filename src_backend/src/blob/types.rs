use std::path::Path;
use std::io::Result;
use utils::hash::Hash;

pub trait Fs {
    fn get_file(&self, path: &Path) -> Option<Vec<u8>>;
    fn save_file(&self, path: &Path, content: &[u8]) -> Result<()>;
}

pub trait KeyValue {
    fn set_blob(&self, content: Vec<u8>) -> Hash;
    fn get_blob(&self, hash: &Hash) -> Option<Vec<u8>>;
}