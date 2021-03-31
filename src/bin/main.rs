use ml_server::ThreadPool;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

const ADDRESS: &str = "127.0.0.1:7878";

const BUFFER_LEN: usize = 1024;

const THREAD_POOL_SIZE: usize = 4;

const LANDING_PAGE: &str = "data/index.html";
const NOT_FOUND_PAGE: &str = "data/404.html";

fn landing_page() -> String {
    let contents = fs::read_to_string(LANDING_PAGE).unwrap();

    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    )
}

fn health_check() -> String {
    String::from("HTTP/1.1 200 OK\r\n\r\n")
}

fn unknown_request() -> String {
    let status = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let contents = fs::read_to_string(NOT_FOUND_PAGE).unwrap();

    format!("{}{}", status, contents)
}

fn long_running_request() -> String {
    thread::sleep(Duration::from_secs(5));
    landing_page()
}

fn check_request_path(buffer: [u8; BUFFER_LEN], path: &str) -> bool {
    buffer.starts_with(format!("GET {} HTTP/1.1\r\n", path).as_bytes())
}

fn get_response(buffer: [u8; BUFFER_LEN]) -> String {
    if check_request_path(buffer, "/long") {
        long_running_request()
    } else if check_request_path(buffer, "/ping") {
        health_check()
    } else if check_request_path(buffer, "/") {
        landing_page()
    } else {
        unknown_request()
    }
}

fn handle(mut stream: TcpStream) {
    let mut buffer = [0; BUFFER_LEN];
    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    stream.write_all(get_response(buffer).as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind(ADDRESS).unwrap();

    let pool: ThreadPool = ThreadPool::new(THREAD_POOL_SIZE);

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        pool.execute(|| {
            handle(stream);
        });
    }
}
