use std::env;

mod blob;

use blob::BlobKeyValue;

/*
użycie haszowania dla pliku ...

/123/345/1234567890 - 1024 * 1024 * 1024
*/

fn main() {
    println!("Hello, world!");

    //Wstrzyknąć trzeba parametr dotyczący katalogu z danymi

    if let Some(arg1) = env::args().nth(1) {
        println!("The first argument is {}", &arg1);

        let key_value_manager = BlobKeyValue::new(arg1);

    } else {
        panic!("Brak parametru");       //TODO
    }
}
