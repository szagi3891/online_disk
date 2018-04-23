use filesystem::utils::hash::{Hash, hash_serializer_format};

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct FileSystemNode {
    pub is_dir: bool,
    #[serde(with = "hash_serializer_format")]
    pub hash: Hash,
}

impl FileSystemNode {
    fn new(is_dir: bool, hash: Hash) -> FileSystemNode {
        FileSystemNode {
            is_dir,
            hash
        }
    }

    pub fn new_dir(hash: Hash) -> FileSystemNode {
        FileSystemNode::new(true, hash)
    }
}
