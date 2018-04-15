extern crate crypto;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate hyper;
extern crate futures;
extern crate tokio_core;
extern crate futures_cpupool;

use std::env;
use std::path::Path;

mod filesystem;
mod server_template;
mod server;

use server::start_server;

fn main() {
    println!("Hello, world!");

    if let Some(root_path) = env::args().nth(1) {
        println!("The first argument is {}", &root_path);

        start_server(Path::new(root_path.as_str()));
    } else {
        panic!("Brak parametru");       //TODO
    }
}
