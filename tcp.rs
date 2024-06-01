use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).expect("Failed to read from client!");

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);

    let response = "Hello, Client!".as_bytes();

    stream.write(response).expect("Failed to write response!");
}

pub fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to the address");
    println!("Server listening on specified port");

    for stream in listener.incoming() {
        match stream {
            Ok(stream_obj) => {
                std::thread::spawn(|| handle_client(stream_obj));
            }
            Err(e) => {
                eprintln!("Failed to establish connection. Error {}", e);
            }
        }
    }
}