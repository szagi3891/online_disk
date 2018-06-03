use hyper::{
    self,
    Method
};
                                                                    //TODO - zamienić na slice (z typu Vec<&'a str>)

fn split_path<'a>(req_path: &'a str) -> Vec<&'a str> {
    let mut out = Vec::new();

    for item in req_path.split('/') {
        if item != "" {
            out.push(item);
        }
    }
    return out; 
}

pub struct UrlChunks<'a> {
    method: &'a hyper::Method,
    items: Vec<&'a str>,
}

impl<'a> UrlChunks<'a> {
    pub fn new(method: &'a hyper::Method, req_path: &'a str) -> UrlChunks<'a> {
        UrlChunks {
            method: method,
            items: split_path(req_path)
        }
    }

    pub fn is_post(&self) -> bool {
        self.method == &Method::POST
    }

    pub fn is_get(&self) -> bool {
        self.method == &Method::GET
    }

    pub fn is_index(&self) -> bool {
        self.items.len() == 0
    }

    fn check_chunks<'b>(&self, chunks: &[&'b str]) -> Option<Vec<&'a str>> {
        let mut items_iter = self.items.iter();

        for item in chunks.iter() {
            if let Some(next_item) = items_iter.next() {
                if **item == *(*next_item) {
                    continue;
                }
            }

            return None;
        }

        let mut out = Vec::new();

        for item in items_iter {
            out.push(*item);
        }

        return Some(out);
    }

    pub fn get<'b>(&self, chunks: &[&'b str]) -> Option<Vec<&'a str>> {
        if self.is_get() {
            return self.check_chunks(chunks);
        }

        None
    }

    pub fn post<'b>(&self, chunks: &[&'b str]) -> Option<Vec<&'a str>> {
        if self.is_post() {
            return self.check_chunks(chunks);
        }

        None
    }
}
