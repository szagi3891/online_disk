use futures::{
    self,
    Stream,
    future::Future
};
use hyper::{
    self,
    StatusCode,
    Response,
    Body,
                            //TODO
    /*
    header::ContentType,
    server::{
        Response
    }
    */
};

use server::utils::set_header::{
    set_content_type_html,
    set_content_type_json
};

pub fn response200(body: String) -> impl Future<Item=Response<Body>, Error=hyper::Error> {
    futures::future::ok(
        set_content_type_json(
            Response::builder()
                .status(StatusCode::OK)
                .body(body.into())
                .unwrap()
        )
    )
}

pub fn response400(body: String) -> impl Future<Item=Response<Body>, Error=hyper::Error> {
    futures::future::ok(Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(body.into())
        .unwrap())
}

pub fn response404(body: String) -> impl Future<Item=Response<Body>, Error=hyper::Error> {
    futures::future::ok(
        set_content_type_html(
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(body.into())
                .unwrap()
        )
    )
}

pub fn response500(body: String) -> impl Future<Item=Response<Body>, Error=hyper::Error> {
    futures::future::ok(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(body.into())
        .unwrap())
}

pub fn get_body_vec(body: hyper::Body) -> impl Future<Item=Vec<u8>, Error=hyper::Error> {
    body
        .collect()
        .and_then(move |chunk| {
            let mut buffer: Vec<u8> = Vec::new();
            for i in chunk {
                buffer.append(&mut i.to_vec());
            }
            Ok(buffer)
        })
}
