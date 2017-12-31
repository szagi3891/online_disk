use std::path::Path;
use filesystem::FileSystem;

use futures;
use futures::future::Future;
use hyper;
use hyper::Client;
use hyper::server::{Request, Response};
use server_template::{ServerBase, ServerBaseExtend, Context};

#[derive(Clone)]
struct Server {
}

impl ServerBaseExtend for Server {
    fn call(&self, req: Request, context: Context) -> Box<Future<Item=Response, Error=hyper::Error>> {
        //panic!("TODO");
        //Box::new(futures::future::ok("dada"))

        let mut response = Response::new();
        response.set_body("<form action='/submit'><input text='data' /></form>");
        Box::new(futures::future::ok(response))
    }
}

pub fn start_server(data_path: &Path) {

    let fs = FileSystem::new(data_path);

    let addr = "127.0.0.1:7777";
    let srv_addr = addr.parse().unwrap();
    
    println!("server start {}", addr);

    ServerBase::run(srv_addr, Server{});

    /*
    let content_hash = fs.create_file(&"bla bla bla bla 2111".as_bytes());

    fs.add(&Vec::new(), &fs.current_head(), &"jakis plikads".into(), &content_hash).unwrap();
    */
}