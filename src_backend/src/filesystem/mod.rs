use std::path::Path;

mod blob;
mod data;
mod head;
mod utils;

use filesystem::head::FileSystemHead;
use filesystem::data::FileSystemData;
use filesystem::blob::key_value::BlobKeyValue;
use filesystem::blob::fs::FsIo;

#[derive(Clone)]
pub struct FileSystem {
    head: FileSystemHead,
    data: FileSystemData<BlobKeyValue<FsIo>>,
}

impl FileSystem {
    pub fn new(path: &Path) -> FileSystem {
        FileSystem {
            head: FileSystemHead::new({
                let mut path_head = path.to_path_buf();
                path_head.push("head");
                path_head
            }),
            data: FileSystemData::new(
                BlobKeyValue::new(
                    {
                        let mut path_data = path.to_path_buf();
                        path_data.push("data");
                        path_data
                    },
                    FsIo{}
                )
            )
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
