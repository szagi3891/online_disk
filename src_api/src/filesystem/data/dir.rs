use std::collections::HashMap;
use serde_json;

use filesystem::data::node::FileSystemNode;

#[derive(Serialize, Deserialize, Debug)]
pub struct FileSystemDir {
    files: HashMap<String, FileSystemNode>,
}

impl FileSystemDir {

    pub fn new(data: HashMap<String, FileSystemNode>) -> FileSystemDir {
        FileSystemDir {
            files: data
        }
    }

    pub fn create_empty() -> FileSystemDir {
        FileSystemDir::new(HashMap::new())
    }
    
    fn to_hashmap(self) -> HashMap<String, FileSystemNode> {
        self.files
    }

    pub fn from_blob(content: &[u8]) -> Result<FileSystemDir, ()> {
        let files: HashMap<String, FileSystemNode> = serde_json::from_slice(content).unwrap();

        Ok(FileSystemDir {
            files: files
        })
    }

    pub fn to_blob(&self) -> Vec<u8> {
        serde_json::to_vec(&self.files).unwrap()
    }

    pub fn set_child(&mut self, subdir: &String, node: FileSystemNode) {
        self.files.insert(subdir.clone(), node);
    }

    pub fn get_child(&self, subdir: &String) -> FileSystemNode {
        (*(self.files.get(subdir).unwrap())).clone()
    }

    pub fn add_child(&mut self, new_subdir: &String, content: FileSystemNode) {
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