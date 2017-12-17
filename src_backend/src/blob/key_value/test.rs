use blob::key_value::BlobKeyValue;
use blob::fs_mock::FsMock;

#[test]
fn exploration() {
    let mut blob_key_value = BlobKeyValue::new(
        "Path/Root".to_string(),
        FsMock::new()
    );

    blob_key_value.set_blob(Vec::from("dasdasda"));

    assert_eq!(
        blob_key_value.get_fs().get_log(),
        vec!("save_file Path/Root/d76/9ab/d769abe7ca1d27e4129d5fd5ce137324df12dec2 6461736461736461".to_string())
    );
}
