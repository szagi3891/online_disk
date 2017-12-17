use std::path::Path;
use std::io::Result;

pub trait Fs {
    fn get_file(&mut self, path: &Path) -> Option<Vec<u8>>;
    fn save_file(&mut self, path: &Path, content: &[u8]) -> Result<()>;
}
