use std::path::Path;
use std::io::Result;

pub trait Fs {
    fn get_file(path: &Path) -> Option<Vec<u8>>;
    fn save_file(path: &Path, content: &[u8]) -> Result<()>;
}
