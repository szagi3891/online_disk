use std::path::{Path, PathBuf};
use std::fs::create_dir_all;

mod blob;
pub mod data;
mod head;
pub mod utils;

use filesystem::head::FileSystemHead;
use filesystem::data::FileSystemData;
use filesystem::blob::key_value::BlobKeyValue;
use filesystem::blob::fs::FsIo;
use filesystem::utils::hash::Hash;
use filesystem::data::dir::FileSystemDir;
use filesystem::head::{CurrentHead};

fn create_sub_path(path: &Path, sub_dir: &str) -> PathBuf {
    let mut path_buf = path.to_path_buf();
    path_buf.push(sub_dir);
    path_buf
}

#[derive(Clone)]
pub struct FileSystem {
    head: FileSystemHead,
    data: FileSystemData<BlobKeyValue<FsIo>>,
}

impl FileSystem {
    pub fn new(path: &Path) -> FileSystem {
        let path_head = create_sub_path(path, "head");
        let path_data = create_sub_path(path, "data");

        create_dir_all(&path_head).unwrap();
        create_dir_all(&path_data).unwrap();

        let data = FileSystemData::new(
            BlobKeyValue::new(
                path_data,
                FsIo{}
            )
        );

        let head = FileSystemHead::new(
            path_head,
            &data
        );

        FileSystem {
            head: head,
            data: data
        }
    }

    fn try_replace_head(&self, head: Hash, new_head: Option<Hash>) -> Option<Result<(), ()>> {
        match new_head {
            Some(new_head) => {
                if let Ok(_) = self.head.replace(head, new_head) {
                    return Some(Ok(()));
                }

                None
            },
            None => {
                return Some(Err(()));
            }
        }
    }

    pub fn current_head(&self) -> CurrentHead {
        self.head.current()
    }

    pub fn create_file(&self, data: &[u8]) -> Hash {
        self.data.create_file(data)
    }


    pub fn get_dir(&self, target_path: &[String], target_hash: &Hash) -> Option<FileSystemDir> {
        let current = self.head.current();
        self.data.get_dir(&current.head, target_path, target_hash)
    }

    /*
    //TODO - do przywrócenia
    pub fn create_dir(&self, data: HashMap<String, Hash>) -> Hash {
        self.data.create_dir(data)
    }

    pub fn add(&self, target_path: &[String], target_hash: &Hash, name: &String, content: &Hash) -> Result<(), ()> {
        loop {
            let head = self.head.current_head();
            let new_head = self.data.add(&head, (target_path, target_hash), name, content.clone());
            if let Some(result) = self.try_replace_head(head, new_head) {
                return result;
            }
        }
    }

    pub fn update(&self, target_path: &[String], target_hash: &Hash, name: &String, content: &Hash) -> Result<(), ()> {
        loop {
            let head = self.head.current_head();
            let new_head = self.data.update(&head, (target_path, target_hash), name, content.clone());
            if let Some(result) = self.try_replace_head(head, new_head) {
                return result;
            }
        }
    }
    */

    pub fn add_dir(&self, target_path: &[String], target_hash: &Hash, name: &String) -> Result<(), ()> {
        loop {
            let current = self.head.current();
            let head_new = self.data.add_dir(&current.head, (target_path, target_hash), name);
            if let Some(result) = self.try_replace_head(current.head, head_new) {
                return result;
            }
        }
    }

    pub fn remove(&self, target_path: &[String], target_hash: &Hash, name: &String) -> Result<(), ()> {
        loop {
            let current = self.head.current();
            let head_new = self.data.remove(&current.head, (target_path, target_hash), name);
            if let Some(result) = self.try_replace_head(current.head, head_new) {
                return result;
            }
        }
    }

    pub fn rename(&self, target_path: &[String], target_hash: &Hash, old_name: &String, new_name: &String) -> Result<(), ()> {
        loop {
            let current = self.head.current();
            let head_new = self.data.rename(&current.head, (target_path, target_hash), old_name, new_name);
            if let Some(result) = self.try_replace_head(current.head, head_new) {
                return result;
            }
        }
    }
}
