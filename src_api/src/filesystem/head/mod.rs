use std::sync::Arc;
use std::sync::RwLock;
use filesystem::utils::hash::Hash;
use std::path::PathBuf;
use filesystem::data::FileSystemData;
use filesystem::blob::key_value::BlobKeyValue;
use filesystem::blob::fs::FsIo;
use chrono::{Utc};
use filesystem::utils::save_file::save_file as save_file_disk;
use filesystem::utils::get_file::get_file;

mod list_file;

fn save_file(path: &PathBuf, couter: &u32, current: &Hash) {
    let now = Utc::now();
    let file_name = format!(
        "head_{}_{:06}.hash",
        now.format("%Y-%C-%y-%H-%M-%S"),
        couter
    );

    let mut file_path = path.clone();
    file_path.push(file_name);

    save_file_disk(file_path.as_path(), &current.to_hex().as_bytes()).unwrap();
}

fn get_count_from_path(path: &PathBuf) -> u32 {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let last_chunk = file_name.split("_").last().unwrap();
    let count = last_chunk.split(".").nth(0).unwrap();

    u32::from_str_radix(count, 10).unwrap()
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CurrentHead { 
    pub counter: u32,
    pub head: Hash,
}

#[derive(Clone)]
pub struct FileSystemHead {
    path: PathBuf,
    inner: Arc<RwLock<CurrentHead>>,
}

impl FileSystemHead {
    fn new_from(path: PathBuf, head: Hash, counter: u32) -> FileSystemHead {
        FileSystemHead {
            path: path,
            inner: Arc::new(RwLock::new(CurrentHead {
                counter: counter,
                head: head
            }))
        }
    }

    pub fn new(path: PathBuf, data: &FileSystemData<BlobKeyValue<FsIo>>) -> FileSystemHead {

        let list = list_file::list_file(path.as_path());

        if list.len() == 0 {

            let head_empty_dir = data.create_empty_dir();
            let couter = 1;

            save_file(&path, &couter, &head_empty_dir);

            FileSystemHead::new_from(path, head_empty_dir, couter)
        } else {

            let mut last: Option<(u32, PathBuf)> = None;

            for item in list {
                let counter = get_count_from_path(&item);
                
                last = match last {
                    None => Some((counter, item)),
                    Some((prev_counter, prev_path)) => {
                        if counter > prev_counter {
                            Some((counter, item))
                        } else {
                            Some((prev_counter, prev_path))
                        }
                    }
                };
            }

            let (last_count, last_path) = last.unwrap();

            let head_hash_content = get_file(&last_path.as_path()).unwrap();

            let head_hash = Hash::from_string_bytes(&head_hash_content);

            FileSystemHead::new_from(path, head_hash, last_count)
        }
    }

    fn save_file(&self, couter: &u32, current: &Hash) {
        save_file(&self.path, couter, current)
    }

    pub fn replace(&self, prev_head: Hash, next_head: Hash) -> Result<(), ()> {
        let mut current = self.inner.write().unwrap();

                                            //Zmienił się head w czasie od ostatniego pobrania
        if current.head != prev_head {
            return Err(());
        }
                                            //Niepotrzebna aktualizacja
        if current.head == next_head {
            return Ok(());
        }

        let next_counter = current.counter + 1;

        self.save_file(&next_counter, &next_head);

        current.counter = next_counter;
        current.head = next_head;

        Ok(())
    }

    pub fn current(&self) -> CurrentHead {
        let inner = self.inner.read().unwrap();
        inner.clone()
    }
}
