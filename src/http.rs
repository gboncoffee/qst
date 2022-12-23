use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum HttpMethod {
    GET,
    HEAD,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum HttpResponseCode {
    Continue100,
    OK200,
    BadRequest400,
    Forbbiden403,
    NotFound404,
    MethodNotAllowed405,
    ImATeapot418,
    NotImplemented501,
    HttpVersionNotSupported505,
}

impl ToString for HttpResponseCode {
    fn to_string(&self) -> String {
        match self {
            HttpResponseCode::Continue100                      => String::from("100 Continue"),
            HttpResponseCode::OK200                            => String::from("200 Ok"),
            HttpResponseCode::BadRequest400                    => String::from("400 Bad Request"),
            HttpResponseCode::Forbbiden403                     => String::from("403 Forbidden"),
            HttpResponseCode::NotFound404                      => String::from("404 Not Found"),
            HttpResponseCode::MethodNotAllowed405              => String::from("405 Method Not Allowed"),
            HttpResponseCode::ImATeapot418                     => String::from("418 I'm A Teapot"),
            HttpResponseCode::NotImplemented501                => String::from("501 Not Implemented"),
            HttpResponseCode::HttpVersionNotSupported505       => String::from("505 HTTP Version Not Supported"),
        }
    }
}

pub struct HttpRequest {
    pub method: HttpMethod,
    pub fetch: String,
}
}

pub struct HttpResponse {
    pub code: HttpResponseCode,
    pub content: Option<String>,
    pub content_length: Option<usize>,
}
