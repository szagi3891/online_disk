
use hyper;
use hyper::server::{Http, Request, Response, Service};

use std::net::SocketAddr;
use futures_cpupool::CpuPool;

use futures::Future;
use futures::stream::Stream;
use std::path::Path;
use tokio_core::reactor::Handle as TokioHandle;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use tokio_core::reactor::Handle;

pub trait ServerTrait {
    fn call(&self, req: Request, tokio_handle: TokioHandle) -> Box<Future<Item=Response, Error=hyper::Error>>;
}

pub struct Server<T: ServerTrait> {
    tokio_handle: TokioHandle,
    inner: T,
}

impl<T: ServerTrait> Service for Server<T> {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        {
            let uri_path = Path::new(req.path());

            if !uri_path.is_absolute() {
                panic!("Tylko absolutne ścieżki są dozwolone");
            }
        }

        self.inner.call(req, self.tokio_handle.clone())
    }
}

impl<T: ServerTrait + Clone + 'static> Server<T> {
    pub fn run<FBuild>(srv_addr: SocketAddr, build: FBuild) where FBuild: Fn(&Handle) -> T {
        let http = Http::new();
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let inner = build(&handle);

        let listener = TcpListener::bind(&srv_addr, &handle).unwrap();
        let server = listener
            .incoming()
            .for_each(|(sock, addr)| {
                http.bind_connection(&handle, sock, addr, Server {
                    tokio_handle: handle.clone(),
                    inner: inner.clone()
                });

                Ok(())
            });

        core.run(server).unwrap();
    }
}

