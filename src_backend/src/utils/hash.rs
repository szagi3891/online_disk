use std::path::PathBuf;

use utils::hex::to_hex;

#[derive(PartialEq, Hash, Clone, Serialize, Deserialize, Debug)]
pub struct Hash {
    hash: [u8; 20],
}

impl Eq for Hash {}

impl Hash {
    pub fn new(hash: [u8; 20]) -> Hash {
        
        Hash {
            hash: hash
        }
    }

    #[test]
    pub fn new_for_test(test_num: u8) -> Hash {
        Hash::new([
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, test_num
        ])
    }

                                                                //TODO zmienić potem nazwę na from_bytes
    pub fn from_bytes(hash: &[u8]) -> Hash {
        
        if hash.len() != 20 {
            panic!("nieprawidłowa długość {:?}", hash.len());
        }
        
        let mut out = [0; 20];
        out.copy_from_slice(&hash);
        
        Hash {
            hash: out
        }
    }

    pub fn add_to_path(&self, path: &mut PathBuf) {        
        let slice = &self.hash[..];
                                                //TODO - użyć lepszej metody do konwersji na hex
        let out = to_hex(slice);
        assert!(out.len() == 40);
        path.push(out);
    }

    pub fn get_prefix(&self, pos: u8) -> u8 {
        self.hash[pos as usize]
    }

                                                //TODO - dobrze byłoby to zrobić bez tylu alokacji przy serializowaniu danych
    pub fn to_hex(&self) -> String {
        to_hex(&self.hash)
    }

                                                //TODO - zmienić potem znowu na seiralize
    pub fn serialize(&self, out: &mut Vec<u8>) {
        for item in self.hash.iter() {
            out.push(*item);
        }
    }

    pub fn from_string(data: &str) -> Hash {
        let bytes = data.as_bytes();
        
        assert_eq!(bytes.len(), 40);

        let mut out = [
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ];

        use std::str;

        let mut index = 0;

        for chank in bytes.chunks(2) {
            let chunk_str = str::from_utf8(chank).unwrap();
            out[index] = u8::from_str_radix(chunk_str, 16).unwrap();
            index = index + 1;
        }

        Hash::new(out)
    }
}
