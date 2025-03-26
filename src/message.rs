use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
};

use serde::{Deserialize, Serialize};

use crate::graph::VertexId;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "message_type", content = "body")]
pub enum VerifierMessage {
    RequestGraphInfo,
    RequestLockedSolution,
    RevealEdgeOnLockedSolution(VertexId, VertexId),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "message_type", content = "body")]
pub enum ProverMessage {
    GraphInfo {
        adjacency_list: HashMap<VertexId, Vec<VertexId>>,
        colors: Vec<String>,
    },
    LockedSolution {
        labels: HashMap<VertexId, String>,
    },
    VertexColors(VertexColor, VertexColor),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VertexColor {
    color: String,
    nonce: String,
}

pub fn write_message(stream: &mut TcpStream, msg: impl Serialize) -> std::io::Result<()> {
    let msg_str = serde_json::to_vec(&msg)?;

    let msg_len = msg_str.len() as u32;
    stream.write_all(&msg_len.to_be_bytes())?;

    stream.write_all(&msg_str)?;
    stream.flush()?;

    Ok(())
}

pub fn read_message<T: for<'a> Deserialize<'a>>(stream: &mut TcpStream) -> std::io::Result<T> {
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes)?;
    let msg_len = u32::from_be_bytes(len_bytes) as usize;

    let mut msg_str = vec![0u8; msg_len];
    stream.read_exact(&mut msg_str)?;

    let msg = serde_json::from_slice(&msg_str)?;

    Ok(msg)
}
