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

fn save_file(path: PathBuf, couter: &u32, current: &Hash) {
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

#[derive(Clone)]
pub struct FileSystemHead {
    saving: Arc<RwLock<()>>,
    current: Arc<RwLock<(Hash, u32)>>,
}

impl FileSystemHead {
    pub fn new_from(hash: Hash, couter: u32) -> FileSystemHead {
        FileSystemHead {
            saving: Arc::new(RwLock::new(())),
            current: Arc::new(RwLock::new(
                (hash, couter)
            ))
        }        
    }

    pub fn new(path: PathBuf, data: &FileSystemData<BlobKeyValue<FsIo>>) -> FileSystemHead {

        let list = list_file::list_file(path.as_path());

        if list.len() == 0 {

            let hash_empty_dir = data.create_empty_dir();
            let couter = 1;

            save_file(path, &couter, &hash_empty_dir);

            FileSystemHead::new_from(hash_empty_dir, couter)
        } else {

                                                            //TODO - tymczasowe
            if list.len() == 1 {
                let last = list[0].clone();
                println!("LAST {:?}", last);

                let content = get_file(last.as_path()).unwrap();
                let read_hash = Hash::from_string_bytes(&content);

                return FileSystemHead::new_from(read_hash, 1);
            }

            //odczytaj pierwszego hasha ...
            println!("lista plików {:?}", list);

            panic!("TODO");
            //weź najstarczy i wyciągnij hash-a oraz numer kolejny
            panic!("TODO");
        }


        /*
        FileSystemHead {
            current: Arc::new(RwLock::new(
            ))
        }
        */
    }

    pub fn replace(&self, prev_head: Hash, next_head: Hash) -> Result<(), ()> {
        
        panic!("TODO");

        //let inner = self.inner.write().unwrap();
        //inner.replace(prev_head, next_head)
    }

    pub fn current_head(&self) -> Hash {
        let lock = self.current.read().unwrap();
        let &(ref current, _) = &(*lock);
        current.clone()
    }
}
