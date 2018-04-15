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
mod server;

use server::server::start_server;

fn main() {
    println!("Czytam parametry wejściowe");

    if let (Some(root_path), Some(static_path)) = (env::args().nth(1), env::args().nth(2)) {
        start_server(
            Path::new(root_path.as_str()),
            Path::new(static_path.as_str())
        );
    } else {
        panic!("Niewłaściwe parametry wejściowe");
    }
}
