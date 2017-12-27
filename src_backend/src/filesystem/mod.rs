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
use filesystem::data::GetResult;

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

    pub fn get_node(&self, node: &Hash) -> Option<GetResult> {
        self.data.get(node)
    }

    pub fn add(&self, mut target_path: Vec<String>, target_hash: &Hash, name: &String, content: &Hash) -> Result<(), ()> {
        loop {
            let head = self.head.current_head();

            match self.data.add(&head, (&mut target_path, target_hash), name, content.clone()) {
                Some(new_head) => {
                    if let Ok(_) = self.head.replace(head, new_head) {
                        return Ok(());
                    }
                },
                None => {
                    return Err(());
                }
            }
        }
    }

    pub fn put_content(&self, data: &[u8]) -> Hash {
        self.data.put_content(data)
    }

    /*
    Publiczne metody które będzie udostępniała ta struktura:

    0)
    pub fn put_content(&self, data: &[u8]) -> Hash
    
    2)
    pub fn update(&self, target: (&mut Vec<String>, Hash), name: &String, new_content: Hash) -> Result<(), ()>
        --> current_head + pub fn update(&self, node: Hash, target: (&mut Vec<String>, Hash), name: &String, new_content: Hash) -> Option<Hash>

    4)
    pub fn remove(&self, target: (&mut Vec<String>, Hash), name: &String) -> Result<(), ()>
        --> current_head + pub fn remove(&self, node: Hash, target: (&mut Vec<String>, Hash), name: &String) -> Option<Hash>

    5)
    pub fn rename(&self, target: (&mut Vec<String>, Hash), old_name: &String, new_name: &String) -> Result<(), ()>
        --> current_head + pub fn rename(&self, node: Hash, target: (&mut Vec<String>, Hash), old_name: &String, new_name: &String) -> Option<Hash>
    */
}
