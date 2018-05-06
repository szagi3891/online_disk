use futures::{
    self,
    Stream,
    future::Future
};
use hyper::{
    self,
    StatusCode,
    header::ContentType,
    server::{
        Response
    }
};

pub fn response200(body: String) -> Box<Future<Item=Response, Error=hyper::Error>> {
    Box::new(futures::future::ok(
        Response::new()
            .with_header(ContentType::json())
            .with_status(StatusCode::Ok)
            .with_body(body)
    ))

    //https://github.com/polachok/hyper-json-server/blob/master/src/server.rs
}

pub fn response400(body: String) -> Box<Future<Item=Response, Error=hyper::Error>> {
    let mut response = Response::new();
    response.set_status(StatusCode::BadRequest);
    response.set_body(body);
    return Box::new(futures::future::ok(response));
}

pub fn response404(body: String) -> Box<Future<Item=Response, Error=hyper::Error>> {
    let mut response = Response::new();
    response.set_status(StatusCode::NotFound);
    response.set_body(body);
    Box::new(futures::future::ok(response))
}

pub fn response500(body: String) -> Box<Future<Item=Response, Error=hyper::Error>> {
    let mut response = Response::new();
    response.set_status(StatusCode::InternalServerError);
    response.set_body(body);
    Box::new(futures::future::ok(response))
}

pub fn get_body_vec(body: hyper::Body) -> Box<Future<Item=Vec<u8>, Error=hyper::Error>> {
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