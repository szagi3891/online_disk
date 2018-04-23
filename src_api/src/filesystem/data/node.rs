use filesystem::utils::hash::{Hash, hash_serializer_format};

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct FileSystemNode {
    pub isDir: bool,
    #[serde(with = "hash_serializer_format")]
    pub hash: Hash,
}

impl FileSystemNode {
    fn new(isDir: bool, hash: Hash) -> FileSystemNode {
        FileSystemNode {
            isDir,
            hash
        }
    }

    pub fn newDir(hash: Hash) -> FileSystemNode {
        FileSystemNode::new(true, hash)
    }
}
