use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

const ADDRESS: &str = "127.0.0.1:7878";

const BUFFER_LEN: usize = 1024;

const LANDING_PAGE: &str = "data/index.html";

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; BUFFER_LEN];

    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let contents = fs::read_to_string(LANDING_PAGE).unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind(ADDRESS).unwrap();

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        handle_connection(stream);
    }
}
