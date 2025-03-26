use std::net::TcpStream;

use ir_zkp::message::{read_message, write_message, ProverMessage, VerifierMessage};

fn main() {
    println!("I am the verifier");

    let mut stream = TcpStream::connect("127.0.0.1:9876").unwrap();

    write_message(&mut stream, VerifierMessage::RequestGraphInfo).unwrap();

    let response: ProverMessage = read_message(&mut stream).unwrap();

    println!("Server says: {:?}", response);
}
