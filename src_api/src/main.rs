#![feature(type_ascription)]

extern crate crypto;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate hyper;
extern crate futures;
extern crate tokio_core;
extern crate futures_cpupool;

use std::env;
use std::path::{Path, PathBuf};
use std::fs;

mod filesystem;
mod server;

use server::start_server;

/*
fn to_absolute(relative: &Path) -> PathBuf { 
    let relative_path = relative.to_path_buf();
    let mut absolute_path = std::env::current_dir().unwrap();
    absolute_path.push(relative_path);
    absolute_path
}
*/

fn to_absolute(relative: &Path) -> PathBuf { 
    fs::canonicalize(relative).unwrap()
}

fn main() {
    let params = (
        env::args().nth(1),
        env::args().nth(2),
        env::args().nth(3)
    );

    if let (Some(root_path), Some(static_path), Some(addr)) = params {
        start_server(
            &to_absolute(
                &Path::new(root_path.as_str())
            ),
            &to_absolute(
                &Path::new(static_path.as_str())
            ),
            addr
        );
    } else {
        panic!("Niewłaściwe parametry wejściowe");
    }
}
