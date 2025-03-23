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

    pub fn add_vertex(&mut self, v: VertexId) {
        if let Some(_) = self.edges.insert(v, vec![]) {
            println!("Warning: overwrote vertex in graph with ID {}", v);
        }
    }

    pub fn add_edge(&mut self, u: VertexId, v: VertexId) {
        if !self.contains_edge(u, v) {
            if !self.contains_vertex(u) {
                self.add_vertex(u);
            }

            self.edges.get_mut(&u).unwrap().push(v);
        }
    }

    pub fn get_label(&self, u: VertexId) -> Option<String> {
        self.labels.get(&u).cloned()
    }

    pub fn set_label(&mut self, u: VertexId, label: String) {
        self.labels.insert(u, label);
    }

    pub fn contains_vertex(&self, v: VertexId) -> bool {
        self.edges.contains_key(&v)
    }

    pub fn contains_edge(&self, u: VertexId, v: VertexId) -> bool {
        // graph is undirected so check for either u->v or v->u edge
        if let Some(vs) = self.edges.get(&u) {
            vs.contains(&v)
        } else if let Some(us) = self.edges.get(&v) {
            us.contains(&u)
        } else {
            false
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
