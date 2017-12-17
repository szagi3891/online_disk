use std::path::Path;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::ErrorKind;

pub fn get_file(path: &Path) -> Option<Vec<u8>> {
        
    let mut buf = Vec::new();

    let file_opt = OpenOptions::new().read(true).open(&path);

    match file_opt {
        Ok(mut file) => {

            match file.read_to_end(&mut buf) {

                Ok(_) => {
                    return Some(buf);
                },

                Err(err) => {
                    panic!("error read {:?} -> {:?}", path, err.kind());
                }
            }
        },

        Err(err) => {
            
            match err.kind() {
                
                ErrorKind::NotFound => {
                    return None;
                }
                
                _ => {
                    panic!("error in read {:?}", err)
                }
            }
        }
    }
}
