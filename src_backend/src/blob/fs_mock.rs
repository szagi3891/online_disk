use std::path::Path;
use std::io::Result;
use blob::types::Fs;

pub struct FsMock {

}

impl Fs for FsMock {
    fn get_file(path: &Path) -> Option<Vec<u8>> {
        panic!("TODO 111");
    }

    fn save_file(path: &Path, content: &[u8]) -> Result<()> {
        panic!("TODO 222");
    }
}
