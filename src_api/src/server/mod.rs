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
        Request,
        Response
    }
};
use server::utils::{
    static_file::StaticFile,
    server::{
        ServerTrait,
        Server
    }
};
use filesystem::utils::hash::Hash;
use tokio_core::reactor::Handle;
use futures_cpupool::CpuPool;
use serde_json;

                
fn response400(body: String) -> Box<Future<Item=Response, Error=hyper::Error>> {
    let mut response = Response::new();
    response.set_status(StatusCode::BadRequest);
    response.set_body(body);
    return Box::new(futures::future::ok(response));
}

fn response404(body: String) -> Box<Future<Item=Response, Error=hyper::Error>> {
    let mut response = Response::new();
    response.set_status(StatusCode::NotFound);
    response.set_body(body);
    Box::new(futures::future::ok(response))
}

fn response200(body: String) -> Box<Future<Item=Response, Error=hyper::Error>> {
    Box::new(futures::future::ok(
        Response::new()
            .with_header(ContentType::json())
            .with_status(StatusCode::Ok)
            .with_body(body)
    ))

    //https://github.com/polachok/hyper-json-server/blob/master/src/server.rs
}

fn get_body_vec(body: hyper::Body) -> Box<Future<Item=Vec<u8>, Error=hyper::Error>> {
    Box::new(
        body
            .collect()
            .and_then(move |chunk| {
                let mut buffer: Vec<u8> = Vec::new();
                for i in chunk {
                    buffer.append(&mut i.to_vec());
                }
                Ok(buffer)
            })
    )
}

/*
    Tą strukturę zwracać w odpowiedzi na te requesty

    GET /api/head/
    POST /api/add_dir
*/

                                                                    //TODO - zamienić na slice (z typu Vec<&'a str>)

fn split_path<'a>(req_path: &'a str) -> Vec<&'a str> {
    let mut out = Vec::new();

    for item in req_path.split('/') {
        if item != "" {
            out.push(item);
        }
    }
    return out; 
}

struct UrlChunks<'a> {
    method: &'a hyper::Method,
    items: Vec<&'a str>,
}

impl<'a> UrlChunks<'a> {
    fn new(method: &'a hyper::Method, req_path: &'a str) -> UrlChunks<'a> {
        UrlChunks {
            method: method,
            items: split_path(req_path)
        }
    }

    fn is_post_method(&self) -> bool {
        self.method == &Method::Post
    }

    fn is_get_method(&self) -> bool {
        self.method == &Method::Get
    }

    fn is_index(&self) -> bool {
        self.items.len() == 0
    }

    fn check_chunks<'b>(&self, chunks: &[&'b str]) -> Option<Vec<&'a str>> {
        let mut items_iter = self.items.iter();

        for item in chunks.iter() {
            if let Some(next_item) = items_iter.next() {
                if **item == *(*next_item) {
                    continue;
                }
            }

            return None;
        }

        let mut out = Vec::new();

        for item in items_iter {
            out.push(*item);
        }

        return Some(out);
    }

    fn is_get<'b>(&self, chunks: &[&'b str]) -> Option<Vec<&'a str>> {
        if self.is_get_method() {
            return self.check_chunks(chunks);
        }

        None
    }

    fn is_post<'b>(&self, chunks: &[&'b str]) -> Option<Vec<&'a str>> {
        if self.is_post_method() {
            return self.check_chunks(chunks);
        }

        None
    }
}

#[derive(Clone)]
struct ServerApp {
    static_file: StaticFile,
    filesystem: FileSystem,
}

impl ServerTrait for ServerApp {
    fn call(&self, req: Request, _handle: Handle) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let (method, uri, _, _headers, body) = req.deconstruct();

        let req_path_new = uri.path();

        if req_path_new.len() > 1000 {
            return response400("Zapytanie za długie".to_owned());
        }

        let uri_chunks = UrlChunks::new(&method, req_path_new);

        if uri_chunks.is_get_method() && uri_chunks.is_index() {
            return self.static_file.send_file("index.html");
        }

        if let Some(rest) = uri_chunks.is_get(&["static"]) {
            return self.static_file.send_file(&rest.as_slice().join("/"));
        }

        if uri_chunks.is_get(&["api", "head"]).is_some() {
            return response200(
                serde_json::to_string(
                    &self.filesystem.current_head()
                ).unwrap()
            );
        }

        if uri_chunks.is_post(&["api", "add_dir"]).is_some() {
            let filesystem = self.filesystem.clone();

            return Box::new(
                get_body_vec(body).and_then(move |buffer|{

                    #[derive(Serialize, Deserialize, Debug)]
                    struct Post {
                        dir: String,
                        node_hash: String,
                        path: Vec<String>
                    }

                    let result: serde_json::Result<Post> = serde_json::from_slice(&buffer);

                    match result {
                        Ok(post) => {
                            let hash = Hash::from_string(&post.node_hash);

                            let result_add = filesystem.add_dir(
                                &post.path,
                                &hash,
                                &post.dir
                            );

                            return response200(
                                serde_json::to_string(
                                    &filesystem.current_head()
                                ).unwrap()
                            );
                        }
                        Err(_) => {
                            return response400("Problem ze zdekodowaniem parametrów /api/add_dir".to_string());
                        }
                    }
                })
            );
        }

        if uri_chunks.is_post(&["api", "dir", "list"]).is_some() {
            let filesystem = self.filesystem.clone();

            return Box::new(
                get_body_vec(body).and_then(move |buffer|{

                    #[derive(Serialize, Deserialize, Debug)]
                    struct Post {
                        node_hash: String,
                        path: Vec<String>
                    }

                    let result: serde_json::Result<Post> = serde_json::from_slice(&buffer);

                    match result {
                        Ok(post) => {
                            let hash = Hash::from_string(&post.node_hash);

                            let node_content = filesystem.get_dir(
                                &post.path,
                                &hash
                            );

                            if let Some(node_content) = node_content {
                                return response200(
                                    serde_json::to_string(
                                        &node_content
                                    ).unwrap()
                                );
                            }

                            return response404(
                                format!("Nie udało się przeczytać noda {}", hash.to_hex())
                            );
                        }
                        Err(_) => {
                            return response400("Problem ze zdekodowaniem parametrów /api/add_dir".to_string());
                        }
                    }
                })
            );
        }

        response404("404 ...".to_string())
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