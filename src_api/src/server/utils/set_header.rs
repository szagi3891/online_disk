use hyper::Response;
use std::path::Path;
//use hyper::header::CONTENT_TYPE;    // ContentType;
use hyper::Body;

pub fn set_content_type_html(mut resp: Response<Body>) -> Response<Body> {
    resp.headers_mut().insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
    resp
}

pub fn set_content_type_json(mut resp: Response<Body>) -> Response<Body> {
    resp.headers_mut().insert("Content-Type", "application/json; charset=utf-8".parse().unwrap());
    resp
}

//TODO
pub fn set_content_type_png(mut resp: Response<Body>) -> Response<Body> {
    resp.headers_mut().insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
    resp
}

//TODO
pub fn set_content_type_jpen(mut resp: Response<Body>) -> Response<Body> {
    resp.headers_mut().insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
    resp
}

//TODO
pub fn set_content_type_txt(mut resp: Response<Body>) -> Response<Body> {
    resp.headers_mut().insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
    resp
}

pub fn set_header(mut response: Response<Body>, path: &str) -> Response<Body> {
    match Path::new(path).extension() {
        Some(ext) => match ext.to_str() {
            Some("txt")  => set_content_type_txt(response),            //"plaintext",    //ContentType::plaintext() ,
            Some("jpg")  => set_content_type_jpen(response),             //"jpeg", //ContentType::jpeg(),
            Some("png")  => set_content_type_png(response),     //"png",  //ContentType::png(),
            Some(_)      => set_content_type_html(response), //"html", //ContentType::html(),
            None         => set_content_type_html(response), //"html"      //ContentType::html(),
        },
        
        //None => ContentType::html(),
        None => set_content_type_html(response),     //"html",
    }

    //response.header()
    //response.headers_mut().insert(CONTENT_TYPE, ext.parse().unwrap());
}
