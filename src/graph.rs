use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct VertexId(i32);

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

    pub fn vertices(&self) -> impl Iterator<Item = &VertexId> {
        self.edges.keys()
    }

    pub fn edges(&self) -> impl Iterator<Item = (&VertexId, &VertexId)> {
        self.edges
            .iter()
            .flat_map(|(u, vs)| vs.iter().map(move |v| (u, v)))
    }
}
