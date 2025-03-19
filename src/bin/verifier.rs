use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() {
    println!("I am the verifier");

    let mut stream = TcpStream::connect("127.0.0.1:9876").unwrap();

    stream
        .write(b"hi its me peter griffin from family guy.")
        .unwrap();

    let mut buf = [0; 256];
    stream.read(&mut buf).unwrap();

    println!("Server says: {}", String::from_utf8_lossy(&buf));
}
