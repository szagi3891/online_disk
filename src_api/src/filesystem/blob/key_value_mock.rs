use filesystem::blob::types::KeyValue;
use std::cell::RefCell;
use std::collections::HashMap;

use filesystem::utils::hash::Hash;
use filesystem::utils::hash_by_content::hash_by_content;

pub struct BlobKeyValue {
    data: RefCell<HashMap<Hash, Vec<u8>>>
}

impl KeyValue for BlobKeyValue {
    fn set_blob(&self, content: &[u8]) -> Hash {
        let hash = hash_by_content(content);

        let mut content_vec: Vec<u8> = Vec::new();
        content_vec.extend_from_slice(content);

        let old_content = self.data.borrow_mut().insert(hash.clone(), content_vec);

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