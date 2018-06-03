use hyper;
use hyper::Response;
use std::path::{PathBuf, Path};
use futures::{self, Future};
use hyper::{Body, StatusCode};
use server::utils::set_header::set_header;
use tokio_fs::{
    self,
    File
};
use tokio_io::io::read_to_end;
use server::utils::response_helpers::response404;

fn read_file(file: File) -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send> {
    let buff: Vec<u8> = Vec::new();

    Box::new(
        read_to_end(file, buff).and_then(|(_file, buff)|{
            futures::future::ok(
                Response::builder()
                    .status(StatusCode::OK)
                    .body(buff.into())
                    .unwrap()
            )
        }).or_else(|_err| {
            response404("404 file1".into())
        })
    )
}

fn get_file_path(base_dir: &PathBuf, file_path: &str) -> PathBuf {
    let mut path_buf = base_dir.clone();
    path_buf.extend(Path::new(file_path));
    path_buf        
}

fn to_response(base_dir: &PathBuf, file_path: &str) -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send> {
    let full_path = get_file_path(&base_dir, file_path);

    println!("Próbuję przeczytać plik {:?}", full_path);

    let ff = tokio_fs::file::File::open(full_path);
    //hyper::Error::new(hyper::error::Kind::io, None)
    //let ff2 = ff.map_err(|err| err.into());
    //let ff2 = ff.map_err(hyper::Error::from);
    let ff2 = ff.map_err(|e| panic!("dada") /*eprintln!("server error: {}", e)*/ );

    let aa = ff2.and_then(|file|{
            read_file(file)
        });

    Box::new(aa)
    //Box::new(aa
        /* .err_map(|err| {
            //Ok(response404("404 file2".into()))
            hyper::Error::new(hyper::Error::new_io(err))
        })*/
    //)
}

pub fn send_static_file(base_dir: &PathBuf, file_path: &str) -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send> {
    let index_result = to_response(&base_dir, file_path);

    let file_path = file_path.to_string();

    Box::new(
        index_result.and_then(move |mut result|{
            Box::new(
                futures::future::ok(
                    set_header(result, file_path.as_str())
                )
            )
        }).or_else(|_error|{
            response404("404 file3".into())
        })
    )
}
