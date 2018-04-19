use hyper;
use hyper::Response;
use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::Error;
use tokio_core::reactor::Handle;
use futures_cpupool::CpuPool;
use futures::{self, Future, Stream, Sink, Poll, Async};
use hyper::{Chunk, Body, StatusCode};
use server::utils::set_header::set_header;
use std::io::{Read};
use std::{mem};

use futures::sync::mpsc::SendError;

/// A stream that produces Hyper chunks from a file.
struct FileChunkStream(File);
impl Stream for FileChunkStream {
    type Item = Result<Chunk, hyper::Error>;
    type Error = SendError<Self::Item>;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        // TODO: non-blocking read
        let mut buf: [u8; 16384] = unsafe { mem::uninitialized() };
        match self.0.read(&mut buf) {
            Ok(0) => Ok(Async::Ready(None)),
            Ok(size) => Ok(Async::Ready(Some(Ok(
                Chunk::from(buf[0..size].to_owned())
            )))),
            Err(err) => Ok(Async::Ready(Some(Err(hyper::Error::Io(err))))),
        }
    }
}

#[derive(Clone)]
pub struct StaticFile {
    handle: Handle,
    base_dir: PathBuf,
    cpu_pool: CpuPool,
}

impl StaticFile {
    pub fn new(handle: Handle, base_dir: &Path, cpu_pool: CpuPool) -> StaticFile {
        StaticFile {
            handle: handle,
            base_dir: base_dir.to_path_buf(),
            cpu_pool: cpu_pool,
        }
    }

    fn to_response(&self, rest: &str) -> Result<Response, Error> {
        let mut path_buf = self.base_dir.clone();
        path_buf.extend(Path::new(rest));

        println!("Próbuję przeczytać plik {:?}", path_buf);

        let file = match File::open(path_buf) {
            Ok(file) => file,
            Err(err) => return Err(err),
        };

        let (sender, body) = Body::pair();
        self.handle.spawn(
            self.cpu_pool.spawn(
                sender.send_all(FileChunkStream(file))
                    .map(|_| ())
                    .map_err(|_| ())
            )
        );

        let mut res = Response::new();
        res.set_body(body);
        return Ok(res);
    }


    pub fn send_file(&self, file_path: &str) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let index_result = self.to_response(file_path);

        match index_result {
            Ok(mut response) => {
                println!("OK...");
                set_header(&mut response, file_path);
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

}
