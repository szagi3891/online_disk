pub mod utils;

use std::path::{Path, PathBuf};
use filesystem::FileSystem;

use futures::{
    self,
    future::Future
};
use hyper::{
    self,
    Method,
    server::{
        Request,
        Response
    }
};
use server::utils::{
    static_file::StaticFile,
    match_str,
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
    filesystem: FileSystem,
}

impl ServerTrait for ServerApp {
    fn call(&self, req: Request, _handle: Handle) -> Box<Future<Item=Response, Error=hyper::Error>> {
        if req.method() == &Method::Get {
            let req_path = req.path();

            if req_path == "/" {
                return self.static_file.send_file("index.html");
            }

            if let Some(rest) = match_str::match_str(req_path, "/static/") {
                return self.static_file.send_file(rest);
            }

            if let Some(rest) = match_str::match_str(req_path, "/api/") {
                if rest == "head" {
                    let body = json!({
                        "head": self.filesystem.current_head().to_hex()
                    });

                    return Box::new(futures::future::ok(
                        hyper::Response::new()
                            .with_header(hyper::header::ContentType::json())
                            .with_status(hyper::StatusCode::Ok)
                            .with_body(body.to_string())
                    ));

                    //https://github.com/polachok/hyper-json-server/blob/master/src/server.rs
                }
            }
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

    let cpu_pool_file = CpuPool::new(16);
    let filesystem = FileSystem::new(data_path);

    let addr = "127.0.0.1:7777";
    let srv_addr = addr.parse().unwrap();
    
    println!("server start {}", addr);

    Server::run(srv_addr, |handle: &Handle| {
        ServerApp {
            static_file: StaticFile::new(
                handle.clone(),
                Path::new(static_path),
                cpu_pool_file.clone()
            ),
            filesystem: filesystem.clone()
        }
    });

    /*
    let content_hash = fs.create_file(&"bla bla bla bla 2111".as_bytes());
    fs.add(&Vec::new(), &fs.current_head(), &"jakis plikads".into(), &content_hash).unwrap();
    */
}