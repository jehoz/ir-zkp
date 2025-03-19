use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_client(stream: &mut TcpStream) {
    stream.write(b"AYOOOOOOOOOOOOOOOOOOOOOOOO").unwrap();

    let mut buf = [0; 256];
    stream.read(&mut buf).unwrap();

    println!("Client says: {}", String::from_utf8_lossy(&buf));
}

fn main() {
    println!("I am the prover");

    let listener = TcpListener::bind("127.0.0.1:9876").unwrap();

    for stream in listener.incoming() {
        println!("New client connected.");
        handle_client(&mut stream.unwrap());
    }
}
