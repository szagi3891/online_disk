use std::collections::HashMap;
use serde_json;

use utils::hash::Hash;

const CodeFormat: u8 = 102;       //'f'

#[derive(Serialize, Deserialize, Debug)]
pub struct FileSystemDir {
    files: HashMap<String, Hash>,
}

impl FileSystemDir {

    pub fn new_for_test(data: HashMap<String, Hash>) -> FileSystemDir {
        FileSystemDir {
            files: data
        }
    }

    pub fn from_blob(content: &Vec<u8>) -> FileSystemDir {

        if let Some((head, body)) = content.split_first() {
            assert_eq!(*head, CodeFormat);

            let files: HashMap<String, Hash> = serde_json::from_slice(body).unwrap();

            return FileSystemDir {
                files: files
            };
        }

        panic!("Nieprawidłowe odgałęzienie programu");
    }

    pub fn to_blob(&self) -> Vec<u8> {
        let mut json = serde_json::to_vec(&self.files).unwrap();
        let mut out = vec!(CodeFormat);
        out.append(&mut json);
        out
    }

    pub fn set_child(&mut self, subdir: String, target: Hash) {
        self.files.insert(subdir, target);
    }
}