use std::path::Path;
use std::io::Result;
use filesystem::blob::types::Fs;

mod save_file;
mod get_file;

pub struct FsIo {
}

impl Fs for FsIo {
    fn get_file(&self, path: &Path) -> Option<Vec<u8>> {
        get_file::get_file(path)
    }

    fn save_file(&self, path: &Path, content: &[u8]) -> Result<()> {
        save_file::save_file(path, content)
    }
}

