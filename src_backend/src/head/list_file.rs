use std::path::{Path, PathBuf};
use std::fs::read_dir;

pub fn list_file(path: &Path) -> Vec<PathBuf> {
    
    let dir_list = read_dir(path).unwrap();
    let mut out = Vec::new();
    
    for path_item in dir_list {
        
        let item = path_item.unwrap();
        let metadata = item.metadata().unwrap();
        
        if metadata.is_file() {
            out.push(item.path());
        } else {
            panic!("Spodziewano się wyłącznie samych plików w katalogu: {:?}", path);
        }
    }
    
    out
}