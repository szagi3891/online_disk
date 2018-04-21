pub mod utils;

use std::path::{Path, PathBuf};
use filesystem::FileSystem;

use futures::{
    self,
    Stream,
    future::Future
};
use hyper::{
    self,
    Method,
    StatusCode,
    header::ContentType,
    server::{
        self,
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
use serde_json::{
    self,
    Value
};

                
fn response400(body: &'static str) -> Box<Future<Item=Response, Error=hyper::Error>> {
    let mut response = Response::new();
    response.set_status(StatusCode::BadRequest);
    response.set_body(body);
    return Box::new(futures::future::ok(response));
}

fn response404(body: &'static str) -> Box<Future<Item=Response, Error=hyper::Error>> {
    let mut response = Response::new();
    //response.set_body("<form action='/submit'><input text='data' /></form>");
    response.set_body("404 ...");
    Box::new(futures::future::ok(response))
}

fn response200(body: serde_json::Value) -> Box<Future<Item=Response, Error=hyper::Error>> {
    Box::new(futures::future::ok(
        Response::new()
            .with_header(ContentType::json())
            .with_status(StatusCode::Ok)
            .with_body(body.to_string())
    ))

    //https://github.com/polachok/hyper-json-server/blob/master/src/server.rs
}

struct HeadVersion {
    //hash
    //version
}
/*
    Tą strukturę zwracać w odpowiedzi na te requesty

    GET /api/head/
    POST /api/add_dir


*/

#[derive(Clone)]
struct ServerApp {
    static_file: StaticFile,
    filesystem: FileSystem,
}

impl ServerTrait for ServerApp {
    fn call(&self, req: Request, _handle: Handle) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let (method, uri, _, _headers, body) = req.deconstruct();

        let methodGet = &method == &Method::Get;
        let methodPost = &method == &Method::Post;
        let req_path = uri.path();

        if methodGet && req_path == "/" {
            return self.static_file.send_file("index.html");
        }

        if methodGet {
            if let Some(rest) = match_str::match_str(req_path, "/static/") {
                return self.static_file.send_file(rest);
            }
        }

        if let Some(rest) = match_str::match_str(req_path, "/api/") {
            if methodGet && rest == "head" {
                return response200(json!({
                    "head": self.filesystem.current_head().to_hex()
                }));
            }

            if methodPost && rest == "add_dir" {
                println!("Coś odebrałem");

                return Box::new(
                    body
                        .collect()
                        .and_then(move |chunk| {
                            let mut buffer: Vec<u8> = Vec::new();
                            for i in chunk {
                                buffer.append(&mut i.to_vec());
                            }
                            Ok(buffer)
                        })
                        .and_then(move |buffer|{

                            #[derive(Serialize, Deserialize, Debug)]
                            struct Post {
                                dir: String,
                            }

                            let result: serde_json::Result<Post> = serde_json::from_slice(&buffer);

                            match result {
                                Ok(post) => {
                                    return response200(json!({
                                        "status": "ok"
                                    }));
                                }
                                Err(_) => {
                                    return response400("Problem ze zdekodowaniem parametrów /api/add_dir");
                                }
                            }
                        })
                );
            }

            // /api/node/:hash/dir
            /*
                self.filesystem.get_dir(target_path, target_hash)
            */
        }

        response404("404 ...")
    }
}

pub fn start_server(data_path: &PathBuf, static_path: &PathBuf, addr: String) {
    if !data_path.is_absolute() || !static_path.is_absolute() {
        panic!("Oczekiwano absolutnych ścieżek");
    }
    let server_addr = addr.parse().unwrap();
    println!("server start {}", addr);

    let cpu_pool_file = CpuPool::new(16);
    let filesystem = FileSystem::new(data_path);

    Server::run(server_addr, |handle: &Handle| {
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