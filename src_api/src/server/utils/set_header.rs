use hyper::Response;
use std::path::Path;
use hyper::Body;

pub fn set_content_type_html(mut resp: Response<Body>) -> Response<Body> {
    resp.headers_mut().insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
    resp
}

pub fn set_content_type_json(mut resp: Response<Body>) -> Response<Body> {
    resp.headers_mut().insert("Content-Type", "application/json; charset=utf-8".parse().unwrap());
    resp
}

pub fn set_content_type_png(mut resp: Response<Body>) -> Response<Body> {
    resp.headers_mut().insert("Content-Type", "image/png".parse().unwrap());
    resp
}

pub fn set_content_type_jpen(mut resp: Response<Body>) -> Response<Body> {
    resp.headers_mut().insert("Content-Type", "image/jpeg".parse().unwrap());
    resp
}

pub fn set_content_type_txt(mut resp: Response<Body>) -> Response<Body> {
    resp.headers_mut().insert("Content-Type", "text/plain".parse().unwrap());
    resp
}

pub fn set_header(response: Response<Body>, path: &str) -> Response<Body> {
    match Path::new(path).extension() {
        Some(ext) => match ext.to_str() {
            Some("txt")  => set_content_type_txt(response),
            Some("jpg")  => set_content_type_jpen(response),
            Some("png")  => set_content_type_png(response),
            Some(_)      => set_content_type_html(response),
            None         => set_content_type_html(response),
        },

        None => set_content_type_html(response),
    }
}
