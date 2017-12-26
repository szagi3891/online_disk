use std::sync::Arc;
use std::sync::RwLock;
use filesystem::utils::hash::Hash;
use std::path::PathBuf;

struct FileSystemHeadInner {
    head_path: PathBuf,
}

impl FileSystemHeadInner {
    pub fn new(path: PathBuf) -> FileSystemHeadInner {
        FileSystemHeadInner {
            head_path: path
        }
    }

    pub fn replace(&self, prev_head: Hash, next_head: Hash) -> Result<(), ()> {
        panic!("TODO");
    }

    pub fn current_head(&self) -> Hash {
        panic!("TODO")
    }
}

#[derive(Clone)]
pub struct FileSystemHead {
    inner: Arc<RwLock<FileSystemHeadInner>>,
}

impl FileSystemHead {
    pub fn new(path: PathBuf) -> FileSystemHead {
        FileSystemHead {
            inner: Arc::new(RwLock::new(FileSystemHeadInner::new(path)))
        }
    }

    pub fn replace(&self, prev_head: Hash, next_head: Hash) -> Result<(), ()> {
        let inner = self.inner.write().unwrap();
        inner.replace(prev_head, next_head)
    }

    pub fn current_head(&self) -> Hash {
        let inner = self.inner.read().unwrap();
        inner.current_head()
    }
}
