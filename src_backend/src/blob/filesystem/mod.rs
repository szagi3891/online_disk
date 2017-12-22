use blob::types::KeyValue;
use utils::hash::Hash;
use std::collections::HashMap;

mod dir;
mod file;

use self::dir::FileSystemDir;

fn head_vec<T>(list: &mut Vec<T>) -> Option<T> {
    if list.len() < 1 {
        return None
    }

    let body = list.split_off(1);
    let head = list.pop();
    *list = body;
    head
}

pub struct FileSystem<T: KeyValue> {
    key_value: T,
}

impl<T: KeyValue> FileSystem<T> {
    pub fn new(key_value: T) -> FileSystem<T> {
        FileSystem {
            key_value: key_value
        }
    }

    fn modify_mode<TF>(&self, node: Hash, target: (&mut Vec<String>, Hash), modify_node_f: TF) -> Option<Hash>
        where TF : FnOnce(FileSystemDir) -> FileSystemDir {
        let (target_path, target_node) = target;

        if let Some(target_path_head) = head_vec(target_path) {
            let node_content = self.key_value.get_blob(&node).unwrap();         //TODO - pozbyć się unwrap
            let mut node_dir = FileSystemDir::from_blob(&node_content);

            let next_node = node_dir.get_child(&target_path_head);

            self.modify_mode(next_node, (target_path, target_node), modify_node_f)
                .map(move |new_node_hash| {
                    node_dir.set_child(&target_path_head, new_node_hash);
                    self.key_value.set_blob(node_dir.to_blob())
                })
        
        } else {
            let node_content = self.key_value.get_blob(&node).unwrap();         //TODO - pozbyć się unwrap
            let mut node_dir = FileSystemDir::from_blob(&node_content);

            if target_node == node {
                let node_dir = modify_node_f(node_dir);
                Some(self.key_value.set_blob(node_dir.to_blob()))
            } else {
                None
            }
        }
    }

    pub fn update(&self, node: Hash, target: (&mut Vec<String>, Hash), name: &String, new_content: Hash) -> Option<Hash> {
        self.modify_mode(node, target, |mut node_dir: FileSystemDir| {
            node_dir.set_child(name, new_content);
            node_dir
        })
    }
}


#[test]
fn test_update_success() {
    use blob::key_value_mock::BlobKeyValue;

    let key_value_mock = BlobKeyValue::new();

    let hash_self = key_value_mock.set_blob({
        let dir = FileSystemDir::new_for_test({
            let mut map = HashMap::new();
            map.insert("hhh".to_string(), Hash::new_for_test(3));
            map
        });

        dir.to_blob()
    });

    assert_eq!(hash_self, Hash::from_string("f7affcfe684aad73ab0ad3fedb2b528da33b3022"));

    let fs = FileSystem::new(key_value_mock);

    let result = fs.update(
        hash_self.clone(),
        (&mut Vec::new(), hash_self.clone()),
        &"hhh".to_string(),
        Hash::new_for_test(0x50)                                //nowa wartość
    );

    let inner_hash = result.unwrap();
                                                                //nowy hash powinien być inny
    assert_ne!(hash_self, inner_hash);
    assert_eq!(inner_hash, Hash::from_string("ffc872739db509a3109c9c5adcc7b5613ddc7df7"))
}

#[test]
fn test_update_fail_target() {
    use blob::key_value_mock::BlobKeyValue;

    let key_value_mock = BlobKeyValue::new();

    let hash_self = key_value_mock.set_blob({
        let dir = FileSystemDir::new_for_test({
            let mut map = HashMap::new();
            map.insert("hhh".to_string(), Hash::new_for_test(100));
            map
        });

        dir.to_blob()
    });

    assert_eq!(hash_self, Hash::from_string("3ceeaf40f4348b6e1dd6594526c0a963cee75094"));

    let fs = FileSystem::new(key_value_mock);

    let result = fs.update(
        hash_self.clone(),
        (&mut Vec::new(), Hash::new_for_test(0x99)),
        &"hhh".to_string(),
        Hash::new_for_test(0x50)                                //nowa wartość
    );

    assert_eq!(result, None);
}

#[test]
fn test_update_recursion() {
    //TODO
}
