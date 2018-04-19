use hyper::server::Response;
use std::path::Path;
use hyper::header::ContentType;

pub fn set_header(response: &mut Response, path: &str) {
    let ext = match Path::new(path).extension() {
        Some(ext) => match ext.to_str() {
            Some("txt")  => ContentType::plaintext() ,
            Some("jpg")  => ContentType::jpeg(),
            Some("png")  => ContentType::png(),
            //Some("html") => Type::TextHtml,
            Some(_)      => ContentType::html(),
            None         => ContentType::html(),
        },
        
        None => ContentType::html(),
    };

    response.headers_mut().set(ext);
}
