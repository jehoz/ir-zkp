use std::net::TcpStream;

use ir_zkp::{
    graph::Graph,
    message::{read_message, write_message, ProverMessage, VerifierMessage},
};

fn main() {
    println!("I am the verifier");

    let mut stream = TcpStream::connect("127.0.0.1:9876").unwrap();

    write_message(&mut stream, VerifierMessage::RequestGraphInfo).unwrap();

    let response: ProverMessage = read_message(&mut stream).unwrap();

    match response {
        ProverMessage::GraphInfo {
            adjacency_list,
            colors,
        } => {
            let g = Graph::from_adjacency_list(adjacency_list);

            let num_edges = g.edges().count();
            println!("Received graph with {} edges.", num_edges);
            let rounds = {
                let prob_invalid = (num_edges as f32 - 1.0) / (num_edges as f32);
                let negligible = 0.5f32.powi(128);
                (negligible.log(prob_invalid)).ceil() as i32
            };
            println!(
                "Need to perform {} rounds of verification before accepting proof.",
                rounds
            );
        }
        _ => {
            println!("Unexpected response type: {:?}", response)
        }
    }
}
