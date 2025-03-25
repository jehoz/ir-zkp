use std::collections::HashMap;

pub type VertexId = i32;

/// A simple undirected, unweighted graph.
/// Vertices can be labelled, letting us "color" the graph.
pub struct Graph {
    edges: HashMap<VertexId, Vec<VertexId>>,
    labels: HashMap<VertexId, String>,
}

impl Graph {
    /// Creates a new empty graph.
    pub fn new() -> Graph {
        Graph {
            edges: HashMap::new(),
            labels: HashMap::new(),
        }
    }

    /// Constructs a graph from a given adjacency list.
    pub fn from_adjacency_list(adj_list: HashMap<VertexId, Vec<VertexId>>) -> Graph {
        let mut g = Graph::new();
        for (u, v) in adj_list
            .iter()
            .flat_map(|(u, vs)| vs.iter().map(move |v| (*u, *v)))
        {
            g.add_edge(u, v);
        }
        g
    }

    /// Inserts a vertex with specified vertex ID.  
    /// If a vertex with that ID already exists in the graph, this function does nothing.
    pub fn add_vertex(&mut self, v: VertexId) {
        if !self.contains_vertex(v) {
            self.edges.insert(v, vec![]);
        }
    }

    /// Creates an edge between vertices u and v in the graph.  
    /// If either vertex does not yet exist in the graph, it will be added.
    /// If the edge already exists, this function does nothing.
    pub fn add_edge(&mut self, u: VertexId, v: VertexId) {
        if !self.contains_edge(u, v) {
            self.add_vertex(u);
            self.add_vertex(v);
            self.edges.get_mut(&u).unwrap().push(v);
        }
    }

    /// Returns the label for a specified vertex (if one exists).
    pub fn get_label(&self, u: VertexId) -> Option<String> {
        self.labels.get(&u).cloned()
    }

    /// Labels a vertex in the graph.
    /// If the vertex already has a label, it will be overwritten.
    pub fn set_label(&mut self, u: VertexId, label: String) {
        self.labels.insert(u, label);
    }

    /// Returns whether or not the graph contains a vertex with the specified ID.
    pub fn contains_vertex(&self, v: VertexId) -> bool {
        self.edges.contains_key(&v)
    }

    /// Returns whether or not the graph contains an edge between the specified vertices.
    /// Because the graph is undirected, the order of vertices given doesn't matter.
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

    /// Returns an iterator traversing all of the vertices in the graph.
    pub fn vertices(&self) -> impl Iterator<Item = VertexId> + '_ {
        self.edges.keys().copied()
    }

    /// Returns an iterator traversing all of the unique edges in the graph.
    pub fn edges(&self) -> impl Iterator<Item = (VertexId, VertexId)> + '_ {
        self.edges
            .iter()
            .flat_map(|(u, vs)| vs.iter().map(move |v| (*u, *v)))
    }

    pub fn adjacency_list(&self) -> &HashMap<VertexId, Vec<VertexId>> {
        &self.edges
    }
}
