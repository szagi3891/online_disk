use std::path::Path;
use std::io::Result;
use blob::types::Fs;
use utils::hex::to_hex;

pub struct FsMock {
    data: Vec<String>
}

impl FsMock {
    pub fn new() -> FsMock {
        FsMock {
            data: Vec::new()
        }
    }

    pub fn get_log(self) -> Vec<String> {
        self.data
    }
}

impl Fs for FsMock {
    fn get_file(&mut self, path: &Path) -> Option<Vec<u8>> {
        self.data.push(
            format!(
                "get_file {}",
                path.to_str().unwrap()
            )
        );

        Some(Vec::new())
    }

    fn save_file(&mut self, path: &Path, content: &[u8]) -> Result<()> {
        self.data.push(
            format!(
                "save_file {} {}",
                path.to_str().unwrap(),
                to_hex(content)
            )
        );

        Ok(())
    }
}
