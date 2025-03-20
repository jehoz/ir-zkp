use std::collections::{hash_map::Keys, HashMap};

struct VertexId(i32);

pub struct Graph {
    edges: HashMap<VertexId, Vec<VertexId>>,
    labels: HashMap<VertexId, String>,
}

pub struct Vertices<'a> {
    edge_keys: Keys<'a, VertexId, Vec<VertexId>>,
}

impl<'a> Iterator for Vertices<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.edge_keys.next() {
            Some(vid) => Some(vid.0),
            None => None,
        }
    }
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            edges: HashMap::new(),
            labels: HashMap::new(),
        }
    }

    pub fn vertices(&self) -> Vertices {
        Vertices {
            edge_keys: self.edges.keys(),
        }
    }
}
