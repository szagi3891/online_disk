use blob::types::KeyValue;
use utils::hash::Hash;

mod dir;
mod file;

pub struct FileSystem<T: KeyValue> {
    key_value: T,
}

impl<T: KeyValue> FileSystem<T> {
    pub fn new(key_value: T) -> FileSystem<T> {
        FileSystem {
            key_value: key_value
        }
    }

    pub fn update(&self, node: Hash, target: (Vec<String>, Hash), new_child: Hash) -> Hash {
        let (target_path, target_node) = target;

        if let Some((head, body)) = target_path.split_first() {
            
            if body.len() == 0 {
                let node_content = self.key_value.get_blob(&node).unwrap();         //TODO - pozbyć się unwrap
                let node_dir = dir::FileSystemDir::from_blob(&node_content);

                panic!("TODO - do doimplementowania");

            } else {

                //self.key_value.get_blob(node)
                //zdekodowany node powinien być katalogiem
                //w tym katalogu powinniśmy się odwołać do węzła wskazywanym przez zmienną head
                //wywołujemy rekurencyjny update i dostajemy nowego hasha
                //zapisujemy ten nasz aktualny katalog, i generujemy nowego hasha którego następnie zwracamy

                panic!("TODO - to implement");
            }
        } else {
            panic!("nieprawidłowe odgałęzienie");
        }
    }
}


#[test]
fn test_update() {
    use blob::key_value_mock::BlobKeyValue;

    let key_value_mock = BlobKeyValue::new();

    //TODO - napchaj mocka testowymi danymi
    //key_value_mock.set_blob();

    let fs = FileSystem::new(key_value_mock);

    //1 to katalog { hhh : 2 }
    //1 [hhh] 2 3 ---> ma wygenerować nowego hasha

    let result = fs.update(
        Hash::new_for_test(0x01),
        (vec!("hhh".to_string()), Hash::new_for_test(0x02)),
        Hash::new_for_test(0x03)
    );
}
