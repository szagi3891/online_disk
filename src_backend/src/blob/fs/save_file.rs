use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Result;
use std::fs;

pub fn save_file(path: &Path, content: &[u8]) -> Result<()> {
                    //https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create_new

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    let file_opt = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path);

    match file_opt {
        Ok(mut file) => {

            file.write_all(content).unwrap();
            file.flush().unwrap();

            Ok(())
        },
        Err(err) => Err(err),
    }
}