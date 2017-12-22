use blob::types::KeyValue;
use std::cell::RefCell;
use std::collections::HashMap;

use utils::hash::Hash;
use utils::hash_by_content::hash_by_content;

pub struct BlobKeyValue {
    data: RefCell<HashMap<Hash, Vec<u8>>>
}

impl KeyValue for BlobKeyValue {
    fn set_blob(&self, content: Vec<u8>) -> Hash {
        let hash = hash_by_content(&content);
        let old_content = self.data.borrow_mut().insert(hash.clone(), content.clone());

        if let Some(old_inner) = old_content {
            assert_eq!(old_inner, content);
        }
 
        println!("set_blob by {}", hash.to_hex());
        hash
    }

    fn get_blob(&self, hash: &Hash) -> Option<Vec<u8>> {
        println!("get_blob by {}", hash.to_hex());

        let borr = self.data.borrow();

        let result = borr.get(hash);

        match result {
            Some(result) => Some(result.clone()),
            None => None,
        }
    }
}

impl BlobKeyValue {
    pub fn new() -> BlobKeyValue {
        BlobKeyValue {
            data: RefCell::new(HashMap::new())
        }
    }
}