use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::graph::VertexId;

enum VerifierMessage {
    RequestGraphInfo,
    RequestLockedSolution,
    RevealEdgeOnLockedSolution(VertexId, VertexId),
}

enum ProverMessage {
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
struct VertexColor {
    color: String,
    nonce: String,
}
