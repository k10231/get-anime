use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

mod request_data;
use rand::seq::SliceRandom;

fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("{}", req_str);
        }
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: TcpStream) {
    let vec = request_data::notmain().unwrap();
    let vec: Vec<&str> = vec.split(",").collect();
    // println!("{:?}", vec.choose(&mut rand::thread_rng()).unwrap());
    let fmt = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{}\r\n",
        vec.choose(&mut rand::thread_rng()).unwrap()
    );
    // println!("{}", fmt);

    let response = fmt.as_bytes();
    match stream.write(response) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream) {
    handle_read(&stream);
    handle_write(stream);
}
fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Listening for connections on port {}", 8080);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
