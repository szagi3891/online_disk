use std::sync::Arc;
use std::sync::RwLock;
use utils::hash::Hash;

mod blob;
mod data;

struct FileSystemInner {
    //head: Hash,
}

impl FileSystemInner {
    pub fn new() -> FileSystemInner {
        FileSystemInner {
        }
    }

    pub fn transaction(&self, prev_head: Hash, next_head: Hash) -> Result<(), ()> {
        panic!("TODO");
    }

    pub fn current_head(&self) -> Hash {
        panic!("TODO")
    }
}

#[derive(Clone)]
pub struct FileSystem {
    inner: Arc<RwLock<FileSystemInner>>,
}

impl FileSystem {
    pub fn new() -> FileSystem {
        FileSystem {
            inner: Arc::new(RwLock::new(FileSystemInner::new()))
        }
    }

    pub fn transaction(&self, prev_head: Hash, next_head: Hash) -> Result<(), ()> {
        let inner = self.inner.write().unwrap();
        inner.transaction(prev_head, next_head)
    }

    pub fn current_head(&self) -> Hash {
        let inner = self.inner.read().unwrap();
        inner.current_head()
    }
}
