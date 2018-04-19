pub mod utils;

use std::path::{Path, PathBuf};
//use filesystem::FileSystem;

use futures;
use futures::future::Future;
use hyper::{self, Method, StatusCode};
use hyper::server::{Request, Response};
use server::utils::{
    static_file::StaticFile,
    match_str,
    set_header::set_header,
    server::{
        ServerTrait,
        Server
    }
};
use tokio_core::reactor::Handle;
use futures_cpupool::CpuPool;

#[derive(Clone)]
struct ServerApp {
    static_file: StaticFile,
}

impl ServerTrait for ServerApp {
    fn call(&self, req: Request, _handle: Handle) -> Box<Future<Item=Response, Error=hyper::Error>> {
        if req.method() == &Method::Get {
            let req_path = req.path();

            if req_path == "/" {
                let index_result = self.static_file.to_response("index.html");

                match index_result {
                    Ok(mut response) => {
                        println!("OK...");
                        set_header(&mut response, "index.html");
                        return Box::new(futures::future::ok(response));
                    },
                    Err(_err) => {
                        println!("OK... {:?}", _err);

                        let mut resp = Response::new()
                            .with_status(StatusCode::NotFound);
                        return Box::new(futures::future::ok(resp));
                    }
                }
            }

            if let Some(rest) = match_str::match_str(req_path, "/static/") {
                match self.static_file.to_response(rest) {
                    Ok(mut response) => {
                        set_header(&mut response, rest);
                        return Box::new(futures::future::ok(response));
                    },
                    Err(_err) => {
                        let mut resp = Response::new()
                            .with_status(StatusCode::NotFound);
                        return Box::new(futures::future::ok(resp));
                    }
                }
            }
            // static
        }

        let mut response = Response::new();
        //response.set_body("<form action='/submit'><input text='data' /></form>");
        response.set_body("404 ...");
        Box::new(futures::future::ok(response))
    }
}

pub fn start_server(data_path: &PathBuf, static_path: &PathBuf) {

    if !data_path.is_absolute() || !static_path.is_absolute() {
        panic!("Oczekiwano absolutnych ścieżek");
    }

    println!("Static path {:?} {:?}", static_path, Path::new(static_path));

    //let fs = FileSystem::new(data_path);

    let addr = "127.0.0.1:7777";
    let srv_addr = addr.parse().unwrap();
    
    println!("server start {}", addr);

    Server::run(srv_addr, |handle: &Handle| {
        let cpu_pool_file = CpuPool::new(16);

        ServerApp {
            static_file: StaticFile::new(handle.clone(), Path::new(static_path), cpu_pool_file.clone()),
        }
    });

    /*
    let content_hash = fs.create_file(&"bla bla bla bla 2111".as_bytes());
    fs.add(&Vec::new(), &fs.current_head(), &"jakis plikads".into(), &content_hash).unwrap();
    */
}