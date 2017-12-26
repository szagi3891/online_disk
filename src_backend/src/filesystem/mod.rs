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

    /*
    Todo: do zaimplementowania wewnętrzne metody head

    self.head.replace(prev_head, next_head);
    self.head.current_head();


    Publiczne metody które będzie udostępniała ta struktura:

    1)
    pub fn get_dir(&self, node: &Hash) -> FileSystemDir
    
    2)
    pub fn update(&self, target: (&mut Vec<String>, Hash), name: &String, new_content: Hash) -> Result<(), ()>
        --> current_head + pub fn update(&self, node: Hash, target: (&mut Vec<String>, Hash), name: &String, new_content: Hash) -> Option<Hash>

    3)
    pub fn add(&self, target: (&mut Vec<String>, Hash), name: &String, new_content: Hash) -> Result<(), ()>
        --> current_head + pub fn add(&self, node: Hash, target: (&mut Vec<String>, Hash), name: &String, new_content: Hash) -> Option<Hash>

    4)
    pub fn remove(&self, target: (&mut Vec<String>, Hash), name: &String) -> Result<(), ()>
        --> current_head + pub fn remove(&self, node: Hash, target: (&mut Vec<String>, Hash), name: &String) -> Option<Hash>

    5)
    pub fn rename(&self, target: (&mut Vec<String>, Hash), old_name: &String, new_name: &String) -> Result<(), ()>
        --> current_head + pub fn rename(&self, node: Hash, target: (&mut Vec<String>, Hash), old_name: &String, new_name: &String) -> Option<Hash>
    */
}
