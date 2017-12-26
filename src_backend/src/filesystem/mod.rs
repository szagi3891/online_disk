use utils::hash::Hash;
use std::path::Path;

mod blob;
mod data;
mod head;

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

    pub fn transaction(&self, prev_head: Hash, next_head: Hash) -> Result<(), ()> {
        self.head.transaction(prev_head, next_head);
        Ok(())
    }

    pub fn current_head(&self) -> Hash {
        self.head.current_head()
    }
}
