pub mod config;
pub mod http;

use config::Config;
use http::*;
use std::io::Result as IoResult;
use std::{
    fs,
    thread,
    process,
    net::{TcpListener, TcpStream},
    io::Write,
};

static mut THREAD_COUNT: isize = 0;

fn write_tcp_or_bail_out(mut stream: TcpStream, string: String) {
    stream.write_all(string.as_bytes()).unwrap_or_else(|_| {
        eprintln!("Fatal server error: Cannot write to TCP Stream. Bailing out. You're on your own. Good luck.");
        process::exit(1);
    });
}

pub fn respond_http_request(mut stream: TcpStream, default_file: String, err404_file: Option<String>) {
    match HttpRequest::parse_tcp_stream(&mut stream) {
        Ok(request) => {
            match request.match_fetch(&default_file[..]) {
                Ok(fetch) => {
                    let mut count = fetch.chars();
                    count.next().unwrap(); // will never panic as fetch is always ./<stuff>
                    count.next().unwrap();
                    if count.next().unwrap() == '_' {
                        write_tcp_or_bail_out(stream, HttpResponse {
                            code: HttpResponseCode::Forbbiden403,
                            content: None,
                            content_length: None,
                        }.to_string());
                        return;
                    }

                    // actually read the file and send it
                    if let IoResult::Ok(content) = fs::read_to_string(fetch) {
                        let len = content.len();
                        write_tcp_or_bail_out(stream, HttpResponse {
                            code: HttpResponseCode::OK200,
                            content: Some(content),
                            content_length: Some(len),
                        }.to_string());
                    } else {
                        let (content, length) = match err404_file {
                            // if the file is valid, uses it, else fails silently
                            Some(file) => {
                                if let IoResult::Ok(string) = fs::read_to_string(file) {
                                    let len = string.len();
                                    (Some(string), Some(len))
                                } else {
                                    (None, None)
                                }
                            },
                            None => (None, None),
                        };
                        write_tcp_or_bail_out(stream, HttpResponse {
                            code: HttpResponseCode::NotFound404,
                            content: content,
                            content_length: length,
                        }.to_string());
                    }
                },
                Err(response) => write_tcp_or_bail_out(stream, response.to_string()),
            }
        },
        Err(response) => write_tcp_or_bail_out(stream, response.to_string()),
    }
}

/// Server main loop. Receives a config and a `incoming` function that must return
/// `Result<Option<TcpStream>, String>`. The `incoming` must return:
/// - `Ok<Some<TcpStream>>` to signal that a new `TcpStream` was received;
/// - `Ok<None>` to signal the server must stop without errors;
/// - `Err<String>` to signal the server must stop with a error.
///
/// Returns `Ok(())` in case of no errors, `Err<String>` in case of errors within the `incoming`
/// function.
///
/// # Examples
/// ```
/// use qst::config::Config;
/// use qst::serve;
/// use std::net::{TcpStream, TcpListener};
///
/// let mut config = Config::new();
/// let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
/// listener.set_nonblocking(true).unwrap();
/// let mut incoming_iter = listener.incoming();
/// serve(config, ||{
///     match incoming_iter.next() {
///         Some(result_stream) => match result_stream {
///             Err(_) => Err(String::from("Connection failed. Bailing out.")),
///             Ok(stream) => Ok(Some(stream)),
///         },
///         None => Ok(None),
///     }
/// });
/// ```
pub fn serve<F>(config: Config, mut incoming: F) -> Result<(), String>
    where
        F: FnMut() -> Result<Option<TcpStream>, String>,
{
    loop {
        match incoming() {
            Ok(Some(stream)) => {
                // wait the thread counter
                if let Some(max_threads) = config.max_threads {
                    unsafe {
                        loop {
                            if THREAD_COUNT < max_threads as isize {
                                break
                            }
                            thread::yield_now();
                        }
                    }
                }

                unsafe {
                    THREAD_COUNT += 1;
                }

                let default_file = config.default_file.clone();
                let err404_file = match config.err404_file {
                    Some(ref file) => Some(file.clone()),
                    None => None,
                };

                if thread::Builder::new()
                    .spawn(move || {
                        respond_http_request(stream, default_file, err404_file);
                        unsafe { THREAD_COUNT -= 1; }
                    })
                    .is_err()
                {
                    return Err(String::from("ERROR: Unable to spawn new threads."));
                }
            },
            Ok(None) => return Ok(()),
            Err(msg) => return Err(msg),
        };
    }
}

/// Starts a server with a config. Returns Err(String) in case of error.
pub fn start_server(config: Config) -> Result<(), String> {

    let full_addr = format!("{}:{}", config.addr, config.port);

    let listener = match TcpListener::bind(full_addr) {
        IoResult::Ok(listener) => listener,
        IoResult::Err(msg) => {
            // full_addr was moved to TcpListener::bind
            let msg = format!("Unable to bind to {}:{}: {msg}", config.addr, config.port);
            return Err(msg);
        },
    };

    println!(
        "Serving HTTP on {} port {} (http://{}:{})...",
        config.addr,
        config.port,
        config.addr,
        config.port
    );

    let mut iter = listener.incoming();
    if let Some(limit) = config.limit_requests {
        let mut count = 0;
        serve(config, move || {
            if count <= limit {
                count += 1;
                match iter.next() {
                    Some(result_stream) => match result_stream {
                        Err(_) => Err(String::from("Connection failed. Bailing out.")),
                        Ok(stream) => Ok(Some(stream)),
                    },
                    None => Ok(None),
                }
            } else {
                Ok(None)
            }
        })
    } else {
        serve(config, move || {
            match iter.next() {
                Some(result_stream) => match result_stream {
                    Err(_) => Err(String::from("Connection failed. Bailing out.")),
                    Ok(stream) => Ok(Some(stream)),
                },
                None => Ok(None),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;
    use std::time::Duration;
    use std::{sync::mpsc, thread};
    
    #[test]
    fn server_starts_and_quit_with_limit_0() {
        let mut config = Config::new();
        config.limit_requests = Some(0);

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            match start_server(config) {
                Ok(()) => tx.send(Ok(())).unwrap(),
                Err(msg) => tx.send(Err(msg)).unwrap(),
            };
        });

        for _ in 0..1000 {
            thread::sleep(Duration::from_millis(5));
            match rx.try_recv() {
                Ok(Ok(_)) => break,
                Ok(Err(msg)) => panic!("Server crashed with message {msg}."),
                Err(_) => panic!("Server did not stop within the 5 second timeout."),
            };
        }
    }
}
