use std::io::Result as IoResult;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    HEAD,
}

#[derive(Debug, PartialEq)]
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
            HttpResponseCode::Continue100 => String::from("100 Continue"),
            HttpResponseCode::OK200 => String::from("200 Ok"),
            HttpResponseCode::BadRequest400 => String::from("400 Bad Request"),
            HttpResponseCode::Forbbiden403 => String::from("403 Forbidden"),
            HttpResponseCode::NotFound404 => String::from("404 Not Found"),
            HttpResponseCode::MethodNotAllowed405 => String::from("405 Method Not Allowed"),
            HttpResponseCode::ImATeapot418 => String::from("418 I'm A Teapot"),
            HttpResponseCode::NotImplemented501 => String::from("501 Not Implemented"),
            HttpResponseCode::HttpVersionNotSupported505 => {
                String::from("505 HTTP Version Not Supported")
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub fetch: String,
}

impl HttpRequest {
    pub fn parse_from_lines_iterator<F>(mut iter: F) -> Result<HttpRequest, HttpResponse>
    where
        F: Iterator<Item = IoResult<String>>,
    {
        let (method, fetch) = if let Some(Ok(line)) = iter.next() {
            let mut line_iter = line.split_ascii_whitespace();
            if let Some(method) = line_iter.next() {
                if let Some(fetch) = line_iter.next() {
                    (method.to_string(), fetch.to_string())
                } else {
                    return Err(HttpResponse::bad_request_400());
                }
            } else {
                return Err(HttpResponse::bad_request_400());
            }
        } else {
            return Err(HttpResponse::bad_request_400());
        };

        let method = match &method[..] {
            "GET" => HttpMethod::GET,
            "HEAD" => HttpMethod::HEAD,
            _ => {
                return Err(HttpResponse {
                    code: HttpResponseCode::NotImplemented501,
                    content: None,
                    content_length: None,
                })
            }
        };
        let fetch = fetch.to_string();
        Ok(HttpRequest { method, fetch })
    }

    pub fn parse_tcp_stream(stream: &mut TcpStream) -> Result<HttpRequest, HttpResponse> {
        let stream_reader = BufReader::new(stream);
        HttpRequest::parse_from_lines_iterator(stream_reader.lines())
    }

    /// Returns the correct path to fetch based on the fetch from a request. Will always be based
    /// uppon the current working directory, starting with `./`.
    ///
    /// If the path is impossible or insecure in a Linux system (i.e., contains "..", "//", or ends
    /// with "/"), returns Err with a proper response to that.
    ///
    /// If the fetch is `/`, returns `default`.
    ///
    /// # Examples:
    /// ```
    /// use qst::http::*;
    /// let request = HttpRequest {
    ///     method: HttpMethod::GET,
    ///     fetch: String::from("/"),
    /// };
    /// assert_eq!("./index.html", HttpRequest::match_fetch(&request, "index.html").unwrap());
    /// ```
    pub fn match_fetch(&self, default: &str) -> Result<String, HttpResponse> {
        if self.fetch == "/" {
            Ok(format!("./{default}"))
        } else if self.fetch == "//coffee" {
            let content = String::from(
                "\
<!DOCTYPE html>
<html lang=\"en\">
    <head>
        <meta charset=\"utf-8\">
        <title>QST Teapots Inc.</title>
    </head>

    <body>
        <h1>418 - I'm a Teapot</h1>
        <p>
        The server refuses to brew coffee because it is a teapot.
        </p>
    </body>
</html>
",
            );
            let content_len = content.len();
            Err(HttpResponse {
                code: HttpResponseCode::ImATeapot418,
                content: Some(content),
                content_length: Some(content_len),
            })
        } else if self.fetch.find("//").is_some()
            || self.fetch.find("..").is_some()
            || self.fetch.ends_with('/')
        {
            Err(HttpResponse {
                code: HttpResponseCode::Forbbiden403,
                content: None,
                content_length: None,
            })
        } else {
            let fetch = self.fetch.replace("%20", " ");
            if fetch.starts_with('/') {
                Ok(format!(".{}", fetch))
            } else {
                Ok(format!("./{}", fetch))
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct HttpResponse {
    pub code: HttpResponseCode,
    pub content: Option<String>,
    pub content_length: Option<usize>,
}

impl HttpResponse {
    fn bad_request_400() -> HttpResponse {
        HttpResponse {
            code: HttpResponseCode::BadRequest400,
            content: None,
            content_length: None,
        }
    }
}

impl ToString for HttpResponse {
    fn to_string(&self) -> String {
        // add statusline
        let mut http_response = String::from("HTTP/1.1 ");
        http_response.push_str(&self.code.to_string()[..]);
        http_response.push('\r');
        http_response.push('\n');

        // add content_length if applicable
        if let Some(length) = self.content_length {
            http_response.push_str(&format!("Content-Length: {length}\r\n\r\n")[..])
        }

        // add content if applicable
        if let Some(content) = &self.content {
            http_response.push_str(&format!("{content}\r\n")[..]);
        }

        // end and return response
        http_response.push('\r');
        http_response.push('\n');

        http_response
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parser_returns_request_on_valid() {
        let request = vec![IoResult::Ok(String::from("GET / HTTP/1.1"))];
        let response = HttpRequest::parse_from_lines_iterator(request.into_iter()).unwrap();
        assert_eq!(
            response,
            HttpRequest {
                method: HttpMethod::GET,
                fetch: String::from("/"),
            }
        );

        let request = vec![IoResult::Ok(String::from("GET /index.html"))];
        let response = HttpRequest::parse_from_lines_iterator(request.into_iter()).unwrap();
        assert_eq!(
            response,
            HttpRequest {
                method: HttpMethod::GET,
                fetch: String::from("/index.html"),
            }
        );

        let request = vec![
            IoResult::Ok(String::from("GET / HTTP/1.0")),
            IoResult::Ok(String::from("Host: pudim.com.br")),
        ];
        let response = HttpRequest::parse_from_lines_iterator(request.into_iter()).unwrap();
        assert_eq!(
            response,
            HttpRequest {
                method: HttpMethod::GET,
                fetch: String::from("/"),
            }
        );

        let request = vec![IoResult::Ok(String::from("HEAD /index.html"))];
        let response = HttpRequest::parse_from_lines_iterator(request.into_iter()).unwrap();
        assert_eq!(
            response,
            HttpRequest {
                method: HttpMethod::HEAD,
                fetch: String::from("/index.html"),
            }
        );
    }

    #[test]
    fn parser_returns_bad_request_on_invalid() {
        let request = vec![IoResult::Ok(String::from("GET"))];
        let response = HttpRequest::parse_from_lines_iterator(request.into_iter()).unwrap_err();
        assert_eq!(HttpResponse::bad_request_400(), response);

        let request = vec![IoResult::Ok(String::from(""))];
        let response = HttpRequest::parse_from_lines_iterator(request.into_iter()).unwrap_err();
        assert_eq!(HttpResponse::bad_request_400(), response);

        let request = vec![];
        let response = HttpRequest::parse_from_lines_iterator(request.into_iter()).unwrap_err();
        assert_eq!(HttpResponse::bad_request_400(), response);
    }

    #[test]
    fn parser_returns_not_implemented_on_methods() {
        let request = vec![IoResult::Ok(String::from("POST / HTTP/1.1"))];
        let response = HttpRequest::parse_from_lines_iterator(request.into_iter()).unwrap_err();
        assert_eq!(
            response,
            HttpResponse {
                code: HttpResponseCode::NotImplemented501,
                content: None,
                content_length: None,
            }
        );
    }

    #[test]
    fn response_to_string_creates_correct_responses() {
        let response = HttpResponse {
            code: HttpResponseCode::NotFound404,
            content: None,
            content_length: None,
        };
        assert_eq!(response.to_string(), "HTTP/1.1 404 Not Found\r\n\r\n");

        let response = HttpResponse::bad_request_400();
        assert_eq!(response.to_string(), "HTTP/1.1 400 Bad Request\r\n\r\n");

        let content = "\
<!DOCTYPE html>
<html>
    <head>
    </head>
    <body>
        Hello, World!
    </body>
</html>
";
        let response = HttpResponse {
            code: HttpResponseCode::OK200,
            content: Some(String::from(content)),
            content_length: Some(content.len()),
        };

        assert_eq!(
            response.to_string(),
            String::from(
                "\
HTTP/1.1 200 Ok\r
Content-Length: 99\r
\r
<!DOCTYPE html>
<html>
    <head>
    </head>
    <body>
        Hello, World!
    </body>
</html>
\r
\r
"
            )
        );
    }

    #[test]
    fn http_request_matches_fetch() {
        let forbidden_res = Err(HttpResponse {
            code: HttpResponseCode::Forbbiden403,
            content: None,
            content_length: None,
        });

        let mut request = HttpRequest {
            method: HttpMethod::GET,
            fetch: String::from("/"),
        };

        assert_eq!(
            Ok(String::from("./index.html")),
            request.match_fetch("index.html")
        );

        request.fetch = String::from("/test.js");
        assert_eq!(
            Ok(String::from("./test.js")),
            request.match_fetch("index.html")
        );

        request.fetch = String::from("stuff.css");
        assert_eq!(
            Ok(String::from("./stuff.css")),
            request.match_fetch("index.html")
        );

        request.fetch = String::from("../not_allow.png");
        assert_eq!(forbidden_res, request.match_fetch("index.html"));

        request.fetch = String::from("/not//allow.jpg");
        assert_eq!(forbidden_res, request.match_fetch("index.html"));

        request.fetch = String::from("not_allow/");
        assert_eq!(forbidden_res, request.match_fetch("index.html"));
    }
}
