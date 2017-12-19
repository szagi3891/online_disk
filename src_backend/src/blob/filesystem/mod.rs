use blob::types::KeyValue;
use utils::hash::Hash;
use std::collections::HashMap;

mod dir;
mod file;

use self::dir::FileSystemDir;

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
                let node_dir = FileSystemDir::from_blob(&node_content);

                /*
                    node_dir --- czy ten node zgadza się z haszem docelowego noda --- ?

                    cała funkcja powinna zwracać typ Option<Hash>



                    dodać drugi przypadek testowy, w którym docelowy hasz się niezgadza
                */

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

    key_value_mock.set_blob({
        let dir = FileSystemDir::new_for_test({
            let mut map = HashMap::new();
            map.insert("hhh".to_string(), Hash::new_for_test(3));
            map
        });

        dir.to_blob()
    });

    let fs = FileSystem::new(key_value_mock);

    let result = fs.update(
        Hash::from_string("f7affcfe684aad73ab0ad3fedb2b528da33b3022"),
        (vec!("hhh".to_string()), Hash::new_for_test(0x02)),
        Hash::new_for_test(0x03)                                //nowa wartość
    );

    panic!("TODO - trzeba porównać wyniki z oczekiwanymi wartościami");
}
