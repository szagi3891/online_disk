pub mod utils;

use std::path::{Path, PathBuf};
use filesystem::FileSystem;

use futures::future::Future;
use hyper::{
    self,
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
    },
    url_router::UrlChunks,
    response_helpers::{
        response200,
        response400,
        response404,
        response500,
        get_body_vec
    }
};
use filesystem::utils::hash::Hash;
use tokio_core::reactor::Handle;
use futures_cpupool::CpuPool;
use serde_json;
use serde::{Serialize, Deserialize};

fn try_serialize<T>(data: &T, error_message: &str) -> Box<Future<Item=Response, Error=hyper::Error>> where T: Serialize {
    match serde_json::to_string(data) {
        Ok(data_string) => response200(data_string),
        Err(err) => response500(
            format!(
                "serde_json serialize error in {} --> {}",
                error_message,
                err
            )
        )
    }
}

fn try_decode<'a, T>(
    buffer: &'a Vec<u8>,
    error_message: &str
) -> Result<T, Box<Future<Item=Response, Error=hyper::Error>>> where T: Deserialize<'a> {
    let result: serde_json::Result<T> = serde_json::from_slice(&buffer);

    match result {
        Ok(post) => Ok(post),
        Err(err) => Err(
            response400(
                format!(
                    "serde_json deserialize error in {} --> {}",
                    error_message,
                    err
                )
            )
        ),
    }
}

macro_rules! try_decode_macro {
    ($e:expr) => (
        match $e {
            Ok(post) => post,
            Err(error_response) => return error_response,
        }
    );
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

        if uri_chunks.is_get() && uri_chunks.is_index() {
            return self.static_file.send_file("index.html");
        }

        if let Some(rest) = uri_chunks.get(&["static"]) {
            return self.static_file.send_file(&rest.as_slice().join("/"));
        }

        if uri_chunks.get(&["api", "head"]).is_some() {
            return try_serialize(
                &self.filesystem.current_head(),
                "request GET /api/head"
            );
        }

        if uri_chunks.post(&["api", "add_dir"]).is_some() {
            let filesystem = self.filesystem.clone();

            return Box::new(
                get_body_vec(body).and_then(move |buffer|{

                    #[derive(Serialize, Deserialize, Debug)]
                    struct Post {
                        dir: String,
                        node_hash: String,
                        path: Vec<String>
                    }

                    let post = try_decode_macro!(
                        try_decode::<Post>(&buffer, "request POST /api/add_dir")
                    );

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
                })
            );
        }

        if uri_chunks.post(&["api", "add_empty_file"]).is_some() {
            let filesystem = self.filesystem.clone();

            return Box::new(
                get_body_vec(body).and_then(move |buffer|{

                    #[derive(Serialize, Deserialize, Debug)]
                    struct Post {
                        node_hash: String,
                        path: Vec<String>,
                        file_name: String,
                    }

                    let post = try_decode_macro!(
                        try_decode::<Post>(&buffer, "request POST /api/add_empty_file")
                    );

                    let hash = Hash::from_string(&post.node_hash);
                    let result_add = filesystem.add_file(
                        &post.path,
                        &hash,
                        &post.file_name,
                        &[]
                    );

                    return response200(
                        serde_json::to_string(
                            &filesystem.current_head()
                        ).unwrap()
                    );
                })
            )
        }

        if uri_chunks.post(&["api", "dir", "list"]).is_some() {
            let filesystem = self.filesystem.clone();

            return Box::new(
                get_body_vec(body).and_then(move |buffer|{

                    #[derive(Serialize, Deserialize, Debug)]
                    struct Post {
                        node_hash: String,
                        path: Vec<String>
                    }

                    let post = try_decode_macro!(
                        try_decode::<Post>(&buffer, "request POST /api/dir/list")
                    );

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
}