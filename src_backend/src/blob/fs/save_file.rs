use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Result;
use std::fs;
use std::io::ErrorKind;

use blob::fs::get_file::get_file;

fn save_file_inner(path: &Path, content: &[u8]) -> Result<()> {
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

pub fn save_file(path: &Path, content: &[u8]) -> Result<()> {

    match save_file_inner(path, content) {
        Ok(()) => Ok(()),
        Err(err) => {
            if err.kind() == ErrorKind::AlreadyExists && verify(path, content) {
                return Ok(());
            }

            Err(err)
        }
    }
}

fn verify(path: &Path, content: &[u8]) -> bool {

    match get_file(path) {
        Some(buf) => buf.as_slice() == content,
        None => {
            panic!("Nigdy program nie powinien wejść w to odgałęzienie");
        },
    }
}
