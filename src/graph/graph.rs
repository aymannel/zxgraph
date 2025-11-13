use crate::graph::vertex::VertexIndex;
use crate::graph::{EdgeType, Vertex};
use petgraph::prelude::{EdgeIndex, StableUnGraph};
use petgraph::stable_graph::EdgeReference;
use petgraph::visit::{IntoEdgeReferences, IntoNodeReferences};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Graph {
    max_qubit: usize,
    inputs: HashMap<usize, VertexIndex>,
    outputs: HashMap<usize, VertexIndex>,
    graph: StableUnGraph<Vertex, EdgeType>,
}

impl Graph {
    /// Creates new Graph
    pub fn new(capacity: usize) -> Self {
        Graph {
            max_qubit: capacity,
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            graph: StableUnGraph::with_capacity(capacity, capacity),
        }
    }

    /// Sets the input vertex for a given qubit, overwriting any existing entry.
    ///
    /// Inserts or updates the mapping in `inputs` between the qubit index and its vertex.
    ///
    /// Runs in **O(1)** time.
    pub fn set_input(&mut self, qubit: usize, vertex_index: VertexIndex) {
        self.inputs.insert(qubit, vertex_index);
    }

    /// Sets the output vertex for a given qubit, overwriting any existing entry.
    ///
    /// Inserts or updates the mapping in `outputs` between the qubit index and its vertex.
    ///
    /// Runs in **O(1)** time.
    pub fn set_output(&mut self, qubit: usize, vertex_index: VertexIndex) {
        self.outputs.insert(qubit, vertex_index);
    }

    /// Adds a vertex to the graph.
    ///
    /// Returns the index of the new vertex.
    ///
    /// Runs in **O(1)** time.
    pub fn add_vertex(&mut self, vertex: Vertex) -> VertexIndex {
        self.graph.add_node(vertex)
    }

    /// Adds a vertex with one input and one output to the graph.
    ///
    /// Returns the index of the new vertex.
    ///
    /// Runs in **O(1)** time.
    pub fn add_unary(&mut self, qubit: usize, vertex: Vertex) -> VertexIndex {
        let vertex_index = self.add_vertex(vertex);
        self.set_input(qubit, vertex_index);
        self.set_output(qubit, vertex_index);
        vertex_index
    }

    /// Adds a `Simple` edge between `source` and `target` to the graph.
    ///
    /// Returns the index of the new edge.
    ///
    /// `Graph` allows adding parallel ("duplicate") edges.
    ///
    /// Runs in **O(1)** time.
    pub fn add_edge(&mut self, source: VertexIndex, target: VertexIndex) -> EdgeIndex {
        self.add_edge_of_type(source, target, EdgeType::Simple)
    }

    /// Adds an edge of type `edge_type` between `source` and `target` to the graph.
    ///
    /// Returns the index of the new edge.
    ///
    /// `Graph` allows adding parallel ("duplicate") edges.
    ///
    /// Runs in **O(1)** time.
    pub fn add_edge_of_type(&mut self, source: VertexIndex, target: VertexIndex, edge_type: EdgeType) -> EdgeIndex {
        self.graph.add_edge(source, target, edge_type)
    }

    /// Removes vertex by index from the graph if it exists.
    ///
    /// Each connected edge is also removed.
    ///
    /// Returns the vertex if it exists and `None` if it does not.
    ///
    /// Runs in **O(`num_edges`)** time.
    pub fn remove_vertex(&mut self, index: VertexIndex) -> Option<Vertex> {
        self.graph.remove_node(index)
    }

    /// Remove edge between `source` and `target`.
    ///
    /// Returns the edge type if it exists and `None` if it does not.
    pub fn remove_edge(&mut self, source: VertexIndex, target: VertexIndex) -> Option<EdgeType> {
        self.graph.remove_edge(self.graph
            .find_edge(source, target)
            .expect("No edge found.")
        )
    }

    /// Removes the input for the given qubit.
    ///
    /// Runs in **O(1)** time.
    pub fn remove_input(&mut self, qubit: usize) {
        self.inputs.remove(&qubit);
    }

    /// Removes the output for the given qubit.
    ///
    /// Runs in **O(1)** time.
    pub fn remove_output(&mut self, qubit: usize) {
        self.outputs.remove(&qubit);
    }

    /// Returns an immutable reference to a vertex by index
    pub fn vertex(&self, index: VertexIndex) -> Option<&Vertex> {
        self.graph.node_weight(index)
    }

    /// Returns a mutable reference to a vertex by its index.
    pub fn vertex_mut(&mut self, index: VertexIndex) -> Option<&mut Vertex> {
        self.graph.node_weight_mut(index)
    }

    /// Returns the index of the edge connecting `source` and `target`, if it exists.
    ///
    /// Runs in **O(`degree`)** time, where *`degree`* is the degree of the searched vertex.
    pub fn edge_index(&self, source: VertexIndex, target: VertexIndex) -> Option<EdgeIndex> {
        self.graph.find_edge(source, target)
    }

    /// Returns an iterator over all immutable vertex references in the graph.
    ///
    /// Runs in **O(n)** time, where *n* is the number of vertices.
    pub fn vertices(&self) -> impl Iterator<Item=&Vertex> {
        self.graph.node_weights()
    }

    /// Returns an iterator over all mutable vertex references in the graph.
    ///
    /// Runs in **O(n)** time, where *n* is the number of vertices.
    pub fn vertices_mut(&mut self) -> impl Iterator<Item=&mut Vertex> {
        self.graph.node_weights_mut()
    }

    /// Returns an iterator over all immutable edge references in the graph.
    ///
    /// Runs in **O(n)** time, where *n* is the number of edges.
    pub fn edges(&self) -> impl Iterator<Item=&EdgeType> {
        self.graph.edge_weights()
    }

    /// Returns an iterator over all mutable edge references in the graph.
    ///
    /// Runs in **O(n)** time, where *n* is the number of edges.
    pub fn edges_mut(&mut self) -> impl Iterator<Item=&mut EdgeType> {
        self.graph.edge_weights_mut()
    }

    /// Returns an iterator over all enumerated vertices in the graph.
    ///
    /// Each item yielded by the iterator is a tuple `(VertexIndex, &Vertex)`,
    ///
    /// Runs in **O(n)** time, where *n* is the number of vertices.
    pub fn enumerate_vertices(&self) -> impl Iterator<Item=(VertexIndex, &Vertex)> {
        self.graph.node_references()
    }

    /// Returns an iterator over all enumerated edges in the graph.
    ///
    /// Each item yielded by the iterator is EdgeReference, from which we can find the
    /// `source` and `target` vertices, as well as the `weight` of the connecting edge.
    ///
    /// Runs in **O(n)** time, where *n* is the number of vertices.
    pub fn enumerate_edges(&self) -> impl Iterator<Item=EdgeReference<'_, EdgeType>> {
        self.graph.edge_references()
    }

    /// Returns the index associated with the given input qubit, if it exists.
    ///
    /// Runs in **O(1)** time.
    pub fn input_index(&self, qubit: usize) -> Option<&VertexIndex> {
        self.inputs.get(&qubit)
    }

    /// Returns the index associated with the given output qubit, if it exists.
    ///
    /// Runs in **O(1)** time.
    pub fn output_index(&self, qubit: usize) -> Option<&VertexIndex> {
        self.outputs.get(&qubit)
    }

    /// Returns an iterator over all input vertex indices in the graph.
    ///
    /// Runs in **O(n)** time, where *n* is the number of inputs.
    pub fn input_indices(&self) -> impl Iterator<Item = &VertexIndex> {
        self.inputs.values()
    }

    /// Returns an iterator over all output vertex indices in the graph.
    ///
    /// Runs in **O(n)** time, where *n* is the number of outputs.
    pub fn output_indices(&self) -> impl Iterator<Item = &VertexIndex> {
        self.outputs.values()
    }

    /// Returns an iterator over all qubits that have associated input vertices in the graph.
    ///
    /// Runs in **O(n)** time, where *n* is the number of inputs.
    pub fn input_qubits(&self) -> impl Iterator<Item = &usize> {
        self.inputs.keys()
    }

    /// Returns an iterator over all qubits that have associated output vertices in the graph.
    ///
    /// Runs in **O(n)** time, where *n* is the number of outputs.
    pub fn output_qubits(&self) -> impl Iterator<Item = &usize> {
        self.outputs.keys()
    }

    /// Returns total number of vertices in the graph.
    ///
    /// Runs in **O(1)** time.
    pub fn num_vertices(&self) -> usize {
        self.graph.node_count()
    }

    /// Returns total number of edges in the graph.
    ///
    /// Runs in **O(1)** time.
    pub fn num_edges(&self) -> usize {
        self.graph.edge_count()
    }

    /// Returns number of input vertices in the graph.
    ///
    /// Runs in **O(1)** time.
    pub fn num_inputs(&self) -> usize {
        self.inputs.len()
    }

    /// Returns number of output vertices in the graph.
    ///
    /// Runs in **O(1)** time.
    pub fn num_outputs(&self) -> usize {
        self.outputs.len()
    }

    /// Returns the maximum qubit
    ///
    /// Runs in **O(1)** time.
    pub fn max_qubit(&self) -> usize {
        self.max_qubit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::VertexBuilder;

    #[test]
    fn can_create_new_graph() {
        let graph = Graph::new(3);
        assert_eq!(graph.num_edges(), 0);
        assert_eq!(graph.num_vertices(), 0);
        assert_eq!(graph.num_inputs(), 0);
        assert_eq!(graph.num_outputs(), 0);
    }

    /// Test Graph::add_vertex(...)
    #[test]
    fn can_add_vertex() {
        let mut graph = Graph::new(1);
        graph.add_vertex(VertexBuilder::x().build());
        assert_eq!(graph.num_vertices(), 1);
    }

    /// Test Graph::add_edge(...)
    #[test]
    fn can_add_edge() {
        let mut graph = Graph::new(1);
        let z = graph.add_vertex(VertexBuilder::z().build());
        let x = graph.add_vertex(VertexBuilder::x().build());
        graph.add_edge(z, x);

        assert_eq!(graph.num_vertices(), 2);
        assert_eq!(graph.num_edges(), 1);
    }
}