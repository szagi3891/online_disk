use std::path::Path;
use std::io::Result;
use blob::types::Fs;
use utils::hex::to_hex;
use std::cell::RefCell;

pub struct FsMock {
    data: RefCell<Vec<String>>
}

impl FsMock {
    pub fn new() -> FsMock {
        FsMock {
            data: RefCell::new(Vec::new())
        }
    }

    pub fn get_log(self) -> Vec<String> {
        self.data.borrow_mut().clone()
    }
}

impl Fs for FsMock {
    fn get_file(&self, path: &Path) -> Option<Vec<u8>> {
        self.data.borrow_mut().push(
            format!(
                "get_file {}",
                path.to_str().unwrap()
            )
        );

        Some(Vec::new())
    }

    fn save_file(&self, path: &Path, content: &[u8]) -> Result<()> {
        self.data.borrow_mut().push(
            format!(
                "save_file {} {}",
                path.to_str().unwrap(),
                to_hex(content)
            )
        );

        Ok(())
    }
}
