use filesystem::blob::types::KeyValue;
use filesystem::utils::hash::Hash;

pub mod dir;
mod file;
mod node;

use self::dir::FileSystemDir;
use self::file::FileSystemFile;
use filesystem::data::node::FileSystemNode;

enum GetResult {
    File(FileSystemFile),
    Dir(FileSystemDir),
}

#[derive(Clone)]
pub struct FileSystemData<T: KeyValue> {
    key_value: T,
}

impl<T: KeyValue> FileSystemData<T> {
    pub fn new(key_value: T) -> FileSystemData<T> {
        FileSystemData {
            key_value: key_value
        }
    }

    fn get_node(&self, node: &Hash) -> Option<GetResult> {
        if let Some(node_content) = self.key_value.get_blob(node) {

            if let Ok(dir) = FileSystemDir::from_blob(&node_content) {
                return Some(GetResult::Dir(dir));
            }

            if let Ok(file) = FileSystemFile::from_blob(&node_content) {
                return Some(GetResult::File(file));
            }
        }

        None
    }

    fn get_node_dir(&self, node: &Hash) -> Option<FileSystemDir> {
        if let Some(GetResult::Dir(dir)) = self.get_node(node) {
            return Some(dir);
        }
        None
    }

    pub fn get_dir(&self, parent: &Hash, target_path: &[String], target_node: &Hash) -> Option<FileSystemDir> {
        if let Some((target_path_head, target_path_rest)) = target_path.split_first() {
            if let Some(GetResult::Dir(dir)) = self.get_node(&parent) {
                let next_node = dir.get_child(&target_path_head);

                return self.get_dir(&next_node.hash, target_path_rest, target_node);
            } else {
                panic!("Spodziewano się katalogu");
            }
        } else {
            if *parent == *target_node {
                if let Some(GetResult::Dir(dir)) = self.get_node(&parent) {
                    return Some(dir);
                }
            }

            None
        }
    }

    pub fn add_dir(&self, node: &Hash, target: (&[String], &Hash), name: &String) -> Option<Hash> {
        self.modify_node(node, target, |mut node_dir: FileSystemDir| {
            let empty_dir_hash = self.key_value.set_blob(
                &FileSystemDir::create_empty().to_blob()
            );

            node_dir.set_child(name, FileSystemNode::new_dir(empty_dir_hash));
            node_dir
        })
    }
    //self.key_value.set_blob(&dir.to_blob())

    fn modify_node<TF>(&self, node: &Hash, target: (&[String], &Hash), modify_node_f: TF) -> Option<Hash>
        where TF : FnOnce(FileSystemDir) -> FileSystemDir {
        let (target_path, target_node) = target;

        if let Some((target_path_head, target_path_rest)) = target_path.split_first() {
            let mut node_dir = self.get_node_dir(&node).unwrap();
            let next_node = node_dir.get_child(&target_path_head);

            if next_node.is_dir {
                return self.modify_node(&next_node.hash, (target_path_rest, target_node), modify_node_f)
                    .map(move |new_node| {
                        node_dir.set_child(&target_path_head, FileSystemNode::new_dir(new_node));
                        self.key_value.set_blob(&node_dir.to_blob())
                    })
            } else {
                panic!("Spodziewano się katalogu");
            }        
        }

        if *target_node == *node {
            let node_dir = modify_node_f(self.get_node_dir(&node).unwrap());
            let new_hash = self.key_value.set_blob(&node_dir.to_blob());
            return Some(new_hash);
        }

        None
    }

    pub fn create_file(&self, data: &[u8]) -> Hash {
        let file = FileSystemFile::new_from_slice(data);
        self.key_value.set_blob(file.ref_data())
    }

    //TODO - do przywrócenia
    /*
    pub fn create_dir(&self, data: HashMap<String, Hash>) -> Hash {
        let dir = FileSystemDir::new(data);
        self.key_value.set_blob(&dir.to_blob())
    }
    */

    //TODO - do przywrócenia
    /*
    pub fn update(&self, node: &Hash, target: (&[String], &Hash), name: &String, new_content: Hash) -> Option<Hash> {
        self.modify_node(node, target, |mut node_dir: FileSystemDir| {
            node_dir.set_child(name, new_content);
            node_dir
        })
    }
    */

    pub fn remove(&self, node: &Hash, target: (&[String], &Hash), name: &String) -> Option<Hash> {
        self.modify_node(node, target, |mut node_dir: FileSystemDir| {
            node_dir.remove_child(name);
            node_dir
        })
    }

    pub fn rename(&self, node: &Hash, target: (&[String], &Hash), old_name: &String, new_name: &String) -> Option<Hash> {
        self.modify_node(node, target, |mut node_dir: FileSystemDir| {
            node_dir.rename_child(old_name, new_name);
            node_dir
        })
    }

    pub fn create_empty_dir(&self) -> Hash {
        let dir = FileSystemDir::create_empty();
        self.key_value.set_blob(&dir.to_blob())
    }
}


#[test]
fn test_update_success() {
    use filesystem::blob::key_value_mock::BlobKeyValue;

    let key_value_mock = BlobKeyValue::new();

    let hash_self = key_value_mock.set_blob({
        let dir = FileSystemDir::new({
            let mut map = HashMap::new();
            map.insert("hhh".to_string(), new_hash_for_test(3));
            map
        });

        &dir.to_blob()
    });

    assert_eq!(hash_self, Hash::from_string("f7affcfe684aad73ab0ad3fedb2b528da33b3022"));

    let fs = FileSystemData::new(key_value_mock);

    let result = fs.update(
        &hash_self,
        (&mut Vec::new(), &hash_self),
        &"hhh".to_string(),
        new_hash_for_test(0x50)                                //nowa wartość
    );

    let inner_hash = result.unwrap();
                                                                //nowy hash powinien być inny
    assert_ne!(hash_self, inner_hash);
    assert_eq!(inner_hash, Hash::from_string("ffc872739db509a3109c9c5adcc7b5613ddc7df7"))
}

#[test]
fn test_update_fail_target() {
    use filesystem::blob::key_value_mock::BlobKeyValue;

    let key_value_mock = BlobKeyValue::new();

    let hash_self = key_value_mock.set_blob({
        let dir = FileSystemDir::new({
            let mut map = HashMap::new();
            map.insert("hhh".to_string(), new_hash_for_test(100));
            map
        });

        &dir.to_blob()
    });

    assert_eq!(hash_self, Hash::from_string("3ceeaf40f4348b6e1dd6594526c0a963cee75094"));

    let fs = FileSystemData::new(key_value_mock);

    let result = fs.update(
        &hash_self,
        (&mut Vec::new(), &new_hash_for_test(0x99)),
        &"hhh".to_string(),
        new_hash_for_test(0x50)                                //nowa wartość
    );

    assert_eq!(result, None);
}

fn new_hash_for_test(test_num: u8) -> Hash {
    Hash::new([
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, test_num
    ])
}
