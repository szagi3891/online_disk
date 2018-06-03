pub mod utils;

use std::path::{Path, PathBuf};
use filesystem::FileSystem;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server};

use futures::future::Future;
use hyper;
use server::utils::{
    url_router::UrlChunks,
    response_helpers::{
        response200,
        response400,
        response404,
        response500,
        get_body_vec
    },
    static_file::send_static_file
};
use filesystem::utils::hash::Hash;
use serde_json;
use serde::{Serialize, Deserialize};


type ResponseFuture = Box<Future<Item=Response<Body>, Error=hyper::Error> + Send>;


fn try_serialize(data: &impl Serialize, error_message: &str) -> ResponseFuture {
    match serde_json::to_string(data) {
        Ok(data_string) => Box::new(response200(data_string)),
        Err(err) => Box::new(response500(
            format!(
                "serde_json serialize error in {} --> {}",
                error_message,
                err
            )
        ))
    }
}

fn try_decode<'a, T>(
    buffer: &'a Vec<u8>,
    error_message: &str
) -> Result<T, ResponseFuture> where T: Deserialize<'a> {
    let result: serde_json::Result<T> = serde_json::from_slice(&buffer);

    match result {
        Ok(post) => Ok(post),
        Err(err) => Err(
            Box::new(response400(
                format!(
                    "serde_json deserialize error in {} --> {}",
                    error_message,
                    err
                )
            ))
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

fn process_request(req: Request<Body>, filesystem: &FileSystem, static_path: &PathBuf) -> ResponseFuture {
    let (req_parts, body) = req.into_parts();

    let req_path_new = req_parts.uri.path();

    {
        let uri_path = Path::new(req_path_new);

        if !uri_path.is_absolute() {
            panic!("Tylko absolutne ścieżki są dozwolone");
        }
    }

    if req_path_new.len() > 1000 {
        return Box::new(response400("Zapytanie za długie".to_owned()));
    }

    let uri_chunks = UrlChunks::new(&req_parts.method, req_path_new);

    if uri_chunks.is_get() && uri_chunks.is_index() {
        return send_static_file(&static_path, "index.html");
    }

    if let Some(rest) = uri_chunks.get(&["static"]) {
        return send_static_file(
            &static_path,
            &rest.as_slice().join("/")
        );
    }

    if uri_chunks.get(&["api", "head"]).is_some() {
        return Box::new(try_serialize(
            &filesystem.current_head(),
            "request GET /api/head"
        ));
    }

    if uri_chunks.post(&["api", "add_dir"]).is_some() {
        let filesystem = filesystem.clone();

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

                return Box::new(response200(
                    serde_json::to_string(
                        &filesystem.current_head()
                    ).unwrap()
                ));
            })
        );
    }

    if uri_chunks.post(&["api", "add_empty_file"]).is_some() {
        let filesystem = filesystem.clone();

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

                return Box::new(response200(
                    serde_json::to_string(
                        &filesystem.current_head()
                    ).unwrap()
                ));
            })
        )
    }

    if uri_chunks.post(&["api", "dir", "list"]).is_some() {
        let filesystem = filesystem.clone();

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
                    return Box::new(response200(
                        serde_json::to_string(
                            &node_content
                        ).unwrap()
                    ));
                }

                return Box::new(response404(
                    format!("Nie udało się przeczytać noda {}", hash.to_hex())
                ));
            })
        );
    }

    Box::new(
        response404("404 ...".into())
    )
}

pub fn start_server(data_path: &PathBuf, static_path: &PathBuf, addr: String) {
    if !data_path.is_absolute() || !static_path.is_absolute() {
        panic!("Oczekiwano absolutnych ścieżek");
    }
    let server_addr = addr.parse().unwrap();

    let static_path = (*static_path).clone();
    let filesystem = FileSystem::new(data_path);

    let server = Server::bind(&server_addr)
        .serve(move || {
            let filesystem = filesystem.clone();
            let static_path = static_path.clone();

            service_fn(
                move |req: Request<Body>| -> ResponseFuture {
                    process_request(req, &filesystem, &static_path)
                }
            )
        })
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    hyper::rt::run(server);
}

