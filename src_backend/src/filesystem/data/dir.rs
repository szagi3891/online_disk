use std::collections::HashMap;
use serde_json;

use filesystem::utils::hash::Hash;

#[derive(Serialize, Deserialize, Debug)]
pub struct FileSystemDir {
    files: HashMap<String, Hash>,
}

impl FileSystemDir {

    pub fn new(data: HashMap<String, Hash>) -> FileSystemDir {
        FileSystemDir {
            files: data
        }
    }

    pub fn to_hashmap(self) -> HashMap<String, Hash> {
        self.files
    }

    pub fn from_blob(content: &[u8]) -> Result<FileSystemDir, ()> {
        let files: HashMap<String, Hash> = serde_json::from_slice(content).unwrap();

        Ok(FileSystemDir {
            files: files
        })
    }

    pub fn to_blob(&self) -> Vec<u8> {
        serde_json::to_vec(&self.files).unwrap()
    }

    pub fn set_child(&mut self, subdir: &String, target: Hash) {
        self.files.insert(subdir.clone(), target);
    }

    pub fn get_child(&self, subdir: &String) -> Hash {
        self.files.get(subdir).unwrap().clone()
    }

    pub fn add_child(&mut self, new_subdir: &String, content: Hash) {
        assert_eq!(self.files.insert(new_subdir.clone(), content), None);
    }

    pub fn remove_child(&mut self, name: &String) {
        let result = self.files.remove(name);
        if result.is_none() {
            panic!("Nieprawidłowe odgałęzienie programu");
        }
    }

    pub fn rename_child(&mut self, old_name: &String, new_name: &String) {
        let result = self.files.remove(old_name);

        if let Some(hash) = result {
            assert_eq!(self.files.insert(new_name.clone(), hash), None);
        } else {
            panic!("Nieprawidłowe odgałęzienie programu");
        }
    }
}