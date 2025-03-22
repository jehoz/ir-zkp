use std::collections::HashMap;

pub type VertexId = i32;

pub struct Graph {
    edges: HashMap<VertexId, Vec<VertexId>>,
    labels: HashMap<VertexId, String>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            edges: HashMap::new(),
            labels: HashMap::new(),
        }
    }

    pub fn vertices(&self) -> impl Iterator<Item = VertexId> + '_ {
        self.edges.keys().copied()
    }

    pub fn edges(&self) -> impl Iterator<Item = (VertexId, VertexId)> + '_ {
        self.edges
            .iter()
            .flat_map(|(u, vs)| vs.iter().map(move |v| (*u, *v)))
    }
}
