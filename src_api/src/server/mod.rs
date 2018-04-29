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
    match_str::{
        convert_to_hash
    },
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

fn url_match_first<'a>(path_chunk: &Vec<&'a str>, pattern: &'a str) -> bool {
    let mut iter = path_chunk.iter();

    let first: Option<&&'a str> = iter.next();
    
    if let Some(first) = first {
        if *first == pattern {
            return true
        }
    }

    false
}

//convert_to_hash

fn url_hash<'a>(path_chunk: &Vec<&'a str>) -> Option<(Hash, Vec<&'a str>)> {
    let mut iter = path_chunk.iter();

    let first: Option<&&'a str> = iter.next();
    
    if let Some(first) = first {
        let hash = convert_to_hash(*first);

        if let Some(hash) = hash {
            let mut out = Vec::new();
            for item in iter {
                out.push(*item);
            }
            return Some((hash, out));

        }
    }

    None
}


fn url_match<'a>(path_chunk: &Vec<&'a str>, pattern: &'a str) -> Option<Vec<&'a str>> {
    let mut iter = path_chunk.iter();

    let first: Option<&&'a str> = iter.next();
    
    if let Some(first) = first {
        if *first == pattern {
            let mut out = Vec::new();
            for item in iter {
                out.push(*item);
            }
            return Some(out);
        }
    }

    None
}

//if let Some(rest) = match_str::match_str(req_path, "/static/") {

#[derive(Clone)]
struct ServerApp {
    static_file: StaticFile,
    filesystem: FileSystem,
}

impl ServerTrait for ServerApp {
    fn call(&self, req: Request, _handle: Handle) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let (method, uri, _, _headers, body) = req.deconstruct();

        let method_get = &method == &Method::Get;
        let method_post = &method == &Method::Post;
        let req_path_new = uri.path();

        if req_path_new.len() > 1000 {
            return response400("Zapytanie za długie".to_owned());
        }

        let path_chunks = split_path(req_path_new);

        if method_get && path_chunks.len() == 0 {
            return self.static_file.send_file("index.html");
        }

        if method_get {
            //if let Some(rest) = match_str::match_str(req_path, "/static/") {
            if let Some(rest) = url_match(&path_chunks, "static") {
                return self.static_file.send_file(&rest.as_slice().join("/"));
            }
        }

        //if let Some(rest) = match_str::match_str(req_path, "/api/") {
        if let Some(rest) = url_match(&path_chunks, "api") {
            if method_get && url_match_first(&rest, "head") {
                return response200(
                    serde_json::to_string(
                        &self.filesystem.current_head()
                    ).unwrap()
                );
                /*
                return response200(json!({
                    "head": self.filesystem.current_head().to_hex()
                }));
                */
            }

            if method_post && url_match_first(&rest, "add_dir") {
                let filesystem = self.filesystem.clone();

                return Box::new(
                    get_body_vec(body).and_then(move |buffer|{

                        #[derive(Serialize, Deserialize, Debug)]
                        struct Post {
                            //target_node: node docelowy
                            //target_path: /path/jakas/adsasdsa
                            dir: String,
                        }

                        let result: serde_json::Result<Post> = serde_json::from_slice(&buffer);

                        match result {
                            Ok(post) => {
                                //self.filesystem.add_dir(target_path, target_hash, name)
                                // target_path - --- root - head ...
                                // target_path [] - pusty slice
                                //name - nowy katalog do dodania


                                //TODO - sparametryzować odpowiednio

                                let target_path: Vec<String> = Vec::new();

                                filesystem.add_dir(
                                    &target_path,
                                    &filesystem.current_head().head,
                                    &post.dir
                                ).unwrap();

                                /*
                                self.filesystem.create_dir(
                                    target_node -- czyli head w tym testowym przypadku
                                    [] - pusta tablice - czyli względem roota nigdzie nie idziemy
                                    string - nowy katalog do utworzenia w środku
                                )
                                */

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

            if method_get {
                //if let Some(node_rest) = match_str::match_str(rest, "dir/") {
                if let Some(node_rest) = url_match(&rest, "dir") {
                    //if let Some((hash, hash_rest)) = match_str::match_hash(node_rest) {
                    if let Some((_hash, _target_path)) = url_hash(&node_rest) {

                        println!("Dostałem request /api/dir {:?} {:?}", &_hash, &_target_path);

                        //TODO - sparametryzować odpowiednio
                        let target_path: Vec<String> = Vec::new();
                        let node_content = self.filesystem.get_dir(
                            &target_path,
                            &self.filesystem.current_head().head
                        );

                        if let Some(node_content) = node_content {
                            return response200(
                                serde_json::to_string(
                                    &node_content
                                ).unwrap()
                            );
                        }

                        return response404(
                            format!("Nie udało się przeczytać noda {}", _hash.to_hex())
                        );
                    }
                }
            }

            // /api/node/:hash/dir
            /*
                self.filesystem.get_dir(target_path, target_hash)
            */
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