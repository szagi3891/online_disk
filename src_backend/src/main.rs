extern crate crypto;

use std::env;

mod blob;
mod utils;

//use blob::BlobKeyValue;

/*
użycie haszowania dla pliku ...

/123/345/1234567890 - 1024 * 1024 * 1024
*/

fn main() {
    println!("Hello, world!");

    //Wstrzyknąć trzeba parametr dotyczący katalogu z danymi

    if let Some(root_path) = env::args().nth(1) {
        println!("The first argument is {}", &root_path);

        use blob::key_value::BlobKeyValue;
        use blob::fs::FsIo;
        use utils::hash::Hash;

        let key_value_manager = BlobKeyValue::new(root_path, FsIo{});

        /*
        key_value_manager.set_blob("dasdas1".to_string().into_bytes());
        key_value_manager.set_blob("dasdas2".to_string().into_bytes());
        key_value_manager.set_blob("dasdas3".to_string().into_bytes());
        */

        /*
        println!(
            "AA {:?}",
            key_value_manager.get_blob(&Hash::new([
                0xb1, 0x44, 0xfd, 0x13, 0xec,
                0xad, 0x26, 0x5c, 0x46, 0xca,
                0x65, 0x24, 0xaf, 0xc7, 0x50,
                0x39, 0xf4, 0x25, 0x01, 0xa2,
            ]))
            .unwrap()
        );
        */

        key_value_manager.set_blob("111 dasdas3 dsdasdasd asdasd asdasdas dasdas dasdasd sadasdas dasdasd adasdas dasdas".to_string().into_bytes());

    } else {
        panic!("Brak parametru");       //TODO
    }
}
