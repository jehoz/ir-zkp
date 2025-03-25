use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::graph::VertexId;

#[derive(Serialize, Deserialize)]
#[serde(tag = "message_type", content = "body")]
pub enum VerifierMessage {
    RequestGraphInfo,
    RequestLockedSolution,
    RevealEdgeOnLockedSolution(VertexId, VertexId),
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct VertexColor {
    color: String,
    nonce: String,
}
