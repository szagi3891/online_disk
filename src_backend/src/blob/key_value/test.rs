use blob::key_value::BlobKeyValue;
use blob::fs_mock::FsMock;

#[test]
fn exploration() {
    let mut blob_key_value = BlobKeyValue::new("Path/Root".to_string(), FsMock{});

    //porównaj dump z tym oczekiwanym

    //blob_key_value.get_fs() --> porównaj assercją z oczekiwanymi danymi

    let blob1 = "dasdasda";
    blob_key_value.set_blob(Vec::from(blob1));

    //blob_key_value.get_fs() ---> dump z operacji porównać z założonym dumpem
    //todo -- 
}
