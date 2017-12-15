
pub struct BlobKeyValue {
    data_path: String,
}

impl BlobKeyValue {
    pub fn new(data_path: String) -> BlobKeyValue {
        BlobKeyValue {
            data_path: data_path
        }
    }
}
