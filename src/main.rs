use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;

const ADDRESS: &str = "127.0.0.1:7878";

const BUFFER_LEN: usize = 1024;


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; BUFFER_LEN];
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

fn main() {
    let listener = TcpListener::bind(ADDRESS).unwrap();

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        handle_connection(stream);
    }
}
