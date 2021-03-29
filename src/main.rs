use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

const ADDRESS: &str = "127.0.0.1:7878";

const BUFFER_LEN: usize = 1024;

const LANDING_PAGE: &str = "data/index.html";
const NOT_FOUND_PAGE: &str = "data/404.html";

fn serve_landing_page() -> String {
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

fn get_formatted_request(path: &str) -> String {
    format!("GET {} HTTP/1.1\r\n", path)
}

fn get_response(buffer: [u8; BUFFER_LEN]) -> String {
    if buffer.starts_with(get_formatted_request("/ping").as_bytes()) {
        health_check()
    } else if buffer.starts_with(get_formatted_request("/").as_bytes()) {
        serve_landing_page()
    } else {
        unknown_request()
    }
}

fn route(mut stream: TcpStream) {
    let mut buffer = [0; BUFFER_LEN];
    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    stream.write(get_response(buffer).as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind(ADDRESS).unwrap();

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        route(stream);
    }
}
