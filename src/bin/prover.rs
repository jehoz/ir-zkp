use std::net::{TcpListener, TcpStream};

use ir_zkp::{
    graph::Graph,
    message::{read_message, write_message, ProverMessage, VerifierMessage},
};

fn handle_client(stream: &mut TcpStream) {
    let g = petersen_graph();

    let ver_msg: VerifierMessage = read_message(stream).unwrap();

    println!("Client says: {:?}", ver_msg);

    match ver_msg {
        VerifierMessage::RequestGraphInfo => {
            let response = ProverMessage::GraphInfo {
                adjacency_list: g.adjacency_list().clone(),
                colors: vec!["red".to_string(), "green".to_string(), "blue".to_string()],
            };

            write_message(stream, response).unwrap();
        }
        VerifierMessage::RequestLockedSolution => todo!(),
        VerifierMessage::RevealEdgeOnLockedSolution(_, _) => todo!(),
    }
}

fn petersen_graph() -> Graph {
    let mut g = Graph::new();
    // outer ring
    g.add_edge(0, 1);
    g.add_edge(1, 2);
    g.add_edge(2, 3);
    g.add_edge(3, 4);
    g.add_edge(4, 0);

    // inner star
    g.add_edge(5, 7);
    g.add_edge(6, 8);
    g.add_edge(7, 9);
    g.add_edge(8, 5);
    g.add_edge(9, 6);

    // connect ring and star
    g.add_edge(0, 5);
    g.add_edge(1, 6);
    g.add_edge(2, 7);
    g.add_edge(3, 8);
    g.add_edge(4, 9);

    g
}

fn main() {
    println!("I am the prover");

    let listener = TcpListener::bind("127.0.0.1:9876").unwrap();

    for stream in listener.incoming() {
        println!("New client connected.");
        handle_client(&mut stream.unwrap());
    }
}
