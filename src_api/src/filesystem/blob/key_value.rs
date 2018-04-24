use std::path::PathBuf;
use std::fmt::Write;

use filesystem::utils::hash_by_content::hash_by_content;
use filesystem::blob::types::Fs;
use filesystem::utils::hash::Hash;
use filesystem::blob::types::KeyValue;

#[derive(Clone)]
pub struct BlobKeyValue<T> where T: Fs {
    data_path: PathBuf,
    fs: T,
}

impl<T> BlobKeyValue<T> where T : Fs {
    pub fn new(data_path: PathBuf, fs: T) -> BlobKeyValue<T> {
        BlobKeyValue {
            data_path: data_path,
            fs: fs
        }
    }

    #[cfg(test)]
    pub fn get_fs(self) -> T {
        self.fs
    }
}

impl<T> KeyValue for BlobKeyValue<T> where T : Fs {
    fn set_blob(&self, content: &[u8]) -> Hash {
        let hash = hash_by_content(content);
        let file_path = create_file_path(&self.data_path, &hash);

        self.fs.save_file(file_path.as_path(), content).unwrap();

        hash
    }

    fn get_blob(&self, hash: &Hash) -> Option<Vec<u8>> {
        let file_path = create_file_path(&self.data_path, hash);
        self.fs.get_file(file_path.as_path())
    }
}

fn create_file_path(data_path: &PathBuf, hash: &Hash) -> PathBuf {
    let (prefix1, prefix2) = extract_prefix_hash(hash);
    let mut data_path = data_path.clone();
    data_path.push(prefix1);
    data_path.push(prefix2);
    data_path.push(hash.to_hex());
    data_path
}

#[test]
fn test_for_create_file_path() {
    let path = PathBuf::from("/aaa/bbb");
    let hash = Hash::new([12, 33, 44, 120, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let out_path = create_file_path(&path, &hash);

    assert_eq!(out_path.to_str().unwrap(), "/aaa/bbb/0c2/12c/0c212c781e000000000000000000000000000000");
}

#[test]
fn test_for_create_file_path2() {
    let path = PathBuf::from("/aaa/bbb");
    let hash = Hash::new([99, 88, 250, 120, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let out_path = create_file_path(&path, &hash);

    assert_eq!(out_path.to_str().unwrap(), "/aaa/bbb/635/8fa/6358fa781e000000000000000000000000000000");
}

fn extract_prefix_hash(hash: &Hash) -> (String, String) {
    let prefix_byte0 = u16::from(hash.get_prefix(0));
    let prefix_byte1 = u16::from(hash.get_prefix(1));
    let prefix_byte2 = u16::from(hash.get_prefix(2));

    let prefix1: u16 = prefix_byte0 << 4 | ((prefix_byte1 & 0xf0) >> 4);
    let prefix2: u16 = ((prefix_byte1 & 0x0f) << 8) | prefix_byte2;

    (to_hex_u16(prefix1), to_hex_u16(prefix2))
}

#[test]
fn test_for_extract_prefix_hash() {
    let hash = Hash::new([12, 33, 44, 120, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let (pref1, pref2) = extract_prefix_hash(&hash);
    assert_eq!(hash.to_hex(), "0c212c781e000000000000000000000000000000");
    assert_eq!(pref1, "0c2");
    assert_eq!(pref2, "12c");
}

#[test]
fn test_for_extract_prefix_hash2() {
    let hash = Hash::new([99, 88, 250, 120, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let (pref1, pref2) = extract_prefix_hash(&hash);
    assert_eq!(hash.to_hex(), "6358fa781e000000000000000000000000000000");
    assert_eq!(pref1, "635");
    assert_eq!(pref2, "8fa");
}

fn to_hex_u16(input: u16) -> String {
    let mut out = String::new();
    write!(&mut out, "{:03x}", input).unwrap();
    out
}

#[test]
fn test_save_blob() {
    use filesystem::blob::key_value::BlobKeyValue;
    use filesystem::blob::fs_mock::FsMock;

    let blob_key_value = BlobKeyValue::new(
        PathBuf::from("Path/Root".to_string()),
        FsMock::new()
    );

    blob_key_value.set_blob(&Vec::from("dasdasda"));

    assert_eq!(
        blob_key_value.get_fs().get_log(),
        vec!("save_file Path/Root/d76/9ab/d769abe7ca1d27e4129d5fd5ce137324df12dec2 6461736461736461".to_string())
    );
}

#[test]
fn test_get_blob() {
    use filesystem::blob::key_value::BlobKeyValue;
    use filesystem::blob::fs_mock::FsMock;

    let blob_key_value = BlobKeyValue::new(
        PathBuf::from("Path/Root".to_string()),
        FsMock::new()
    );

    blob_key_value.get_blob(
        &Hash::new([
            0xd7, 0x69, 0xab, 0xe7, 0xca,
            0x1d, 0x27, 0xe4, 0x12, 0x9d,
            0x5f, 0xd5, 0xce, 0x13, 0x73,
            0x24, 0xdf, 0x12, 0xde, 0xc2
        ])
    );

    assert_eq!(
        blob_key_value.get_fs().get_log(),
        vec!("get_file Path/Root/d76/9ab/d769abe7ca1d27e4129d5fd5ce137324df12dec2".to_string())
    );
}
