use crate::graph::{EdgeType, Vertex, VertexBuilder, VertexType};
use petgraph::prelude::{EdgeIndex, NodeIndex, StableUnGraph};
use petgraph::stable_graph::{EdgeReferences, NodeReferences};
use petgraph::visit::{IntoEdgeReferences, IntoNodeReferences};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Graph {
    base_graph: StableUnGraph<Vertex, EdgeType>,
    inputs: BTreeMap<usize, NodeIndex>,
    outputs: BTreeMap<usize, NodeIndex>,
}

// todo - review methods, which need to be public and which can be made private?
impl Graph {
    /// Creates an empty graph
    pub fn new(capacity: usize) -> Self {
        Graph {
            base_graph: StableUnGraph::with_capacity(capacity, capacity),
            inputs: BTreeMap::new(),
            outputs: BTreeMap::new(),
        }
    }

    // todo - should take NodeIndex of vertex you want to connect to input and the qubit it's then add a wire to it
    // todo - don't specify coords here! only in the builder. use qubit index to determine coords there
    /// Adds new input boundary node
    pub fn add_input(&mut self, qubit: usize) -> NodeIndex {
        assert!(self.input(qubit).is_none(), "Input at qubit {qubit} already exists");
        debug_assert_eq!(self.num_boundaries(), self.inputs.len() + self.outputs.len());
        let input = self.base_graph.add_node(VertexBuilder::b()
            .coords(-1.0, qubit as f64)
            .build()
        );
        self.inputs.insert(qubit, input);
        input
    }

    // todo - should take NodeIndex of vertex you want to connect to output and the qubit it's then add a wire to it
    // todo - don't specify coords here! only in the builder. use qubit index to determine coords there
    /// Adds new output boundary
    pub fn add_output(&mut self, qubit: usize) -> NodeIndex {
        assert!(self.output(qubit).is_none(), "Output at qubit {qubit} already exists");
        debug_assert_eq!(self.num_boundaries(), self.inputs.len() + self.outputs.len());
        let output = self.base_graph.add_node(VertexBuilder::b()
            .coords(1.0, qubit as f64)
            .build()
        );
        self.outputs.insert(qubit, output);
        output
    }

    /// Adds a single wire along the specified qubit. Where a wire is defined as a qubit with no gate (identity).
    pub fn add_wire(&mut self, qubit: usize) {
        assert!(self.inputs.get(&qubit).is_none(), "Input at qubit {qubit} already exists");
        assert!(self.outputs.get(&qubit).is_none(), "Output at qubit {qubit} already exists");
        let input = self.add_input(qubit);
        let output = self.add_output(qubit);
        self.add_edge(input, output);
    }

    /// Adds wires along the specified qubits
    pub fn add_wires(&mut self, qubits: impl IntoIterator<Item=usize>) {
        for qubit in qubits {
            self.add_wire(qubit);
        }
    }

    /// Adds wires along the specified qubits excluding specified qubits
    pub fn add_wires_excluding<I, E>(&mut self, qubits: I, excluded: E)
    where
        I: IntoIterator<Item=usize>,
        E: IntoIterator<Item=usize>,
    {
        let excluded: Vec<_> = excluded.into_iter().collect();
        let filtered_qubits = qubits.into_iter().filter(|i| !excluded.contains(i));
        self.add_wires(filtered_qubits);
    }

    /// Adds new vertex
    pub fn add_vertex(&mut self, vertex: Vertex) -> NodeIndex {
        self.base_graph.add_node(vertex)
    }

    // todo - should be a builder method
    /// Adds new vertex and wire along the specified qubit
    pub fn add_vertex_on_wire(&mut self, qubit: usize, vertex: Vertex) -> NodeIndex {
        let input = self.add_input(qubit);
        let output = self.add_output(qubit);
        let vertex = self.add_vertex(vertex);
        self.add_edge(input, vertex);
        self.add_edge(output, vertex);
        vertex
    }

    /// Adds new edge between two vertices
    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        self.add_edge_of_type(source, target, EdgeType::Simple);
    }

    /// Adds new edge with specified type between two vertices
    pub fn add_edge_of_type(&mut self, source: NodeIndex, target: NodeIndex, edge_type: EdgeType) {
        self.base_graph.add_edge(source, target, edge_type);
    }

    /// Removes vertex with specified index
    pub fn remove_vertex(&mut self, index: NodeIndex) {
        self.base_graph.remove_node(index);
    }

    /// Removes edge between two vertices
    pub fn remove_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        self.base_graph.remove_edge(self.base_graph
            .find_edge(source, target)
            .expect("No edge found.")
        );
    }

    /// Returns vertex by NodeIndex
    pub fn vertex(&self, index: NodeIndex) -> Option<&Vertex> {
        self.base_graph.node_weight(index)
    }

    /// Returns optional Vertex by NodeIndex
    pub fn edge_index(&self, source: NodeIndex, target: NodeIndex) -> Option<EdgeIndex> {
        self.base_graph.find_edge(source, target)
    }

    // todo - exclude inputs/outputs
    /// Returns all vertices in graph, excluding inputs and outputs
    pub fn vertices(&self) -> impl Iterator<Item=&Vertex> {
        self.base_graph.node_weights()
    }

    // todo - exclude inputs/outputs
    /// Returns all vertices in graph, excluding inputs and outputs
    pub fn vertices_mut(&mut self) -> impl Iterator<Item=&mut Vertex> {
        self.base_graph.node_weights_mut().filter(|v| v.vertex_type() != VertexType::B)
    }

    /// Returns all edges in graph
    pub fn edges(&self) -> impl Iterator<Item=&EdgeType> {
        self.base_graph.edge_weights()
    }

    /// Returns total number of vertices, excluding boundaries
    pub fn num_vertices(&self) -> usize {
        self.base_graph.node_count() - self.num_boundaries()
    }

    /// Returns total number of edges
    pub fn num_edges(&self) -> usize {
        self.base_graph.edge_count()
    }

    /// Returns all enumerated vertices in graph, including inputs and outputs
    pub fn enumerate_vertices(&self) -> NodeReferences<'_, Vertex> {
        self.base_graph.node_references()
    }

    /// Returns all enumerated edges in graph, including input and output edges
    pub fn enumerate_edges(&self) -> EdgeReferences<'_, EdgeType> {
        self.base_graph.edge_references()
    }

    /// Returns NodeIndex if input exists at qubit
    pub fn input(&self, qubit: usize) -> Option<NodeIndex> {
        self.inputs.get(&qubit).copied()
    }

    /// Returns NodeIndex if output exists at qubit
    pub fn output(&self, qubit: usize) -> Option<NodeIndex> {
        self.outputs.get(&qubit).copied()
    }

    /// Returns all inputs by qubit in ascending order (BTreeMap is sorted)
    pub fn inputs(&self) -> impl Iterator<Item = (&usize, &NodeIndex)> {
        self.inputs.iter()
    }

    /// Returns all outputs by qubit in ascending order (BTreeMap is sorted)
    pub fn outputs(&self) -> impl Iterator<Item = (&usize, &NodeIndex)> {
        self.outputs.iter()
    }

    /// Returns number of inputs present
    pub fn num_inputs(&self) -> usize {
        self.inputs.len()
    }

    /// Returns number of outputs present
    pub fn num_outputs(&self) -> usize {
        self.outputs.len()
    }

    /// Returns true if self can be composed
    fn is_composable(&self) -> bool {
        self.num_vertices() > 0 && self.vertices().all(|v| v.coords().is_some())
    }

    /// Returns number of vertices of type VertexType::B
    fn num_boundaries(&self) -> usize {
        self.base_graph.node_weights().filter(|v| v.vertex_type() == VertexType::B).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        graph.add_wire(1);

        assert_eq!(graph.num_vertices(), 2);
        assert_eq!(graph.num_inputs(), 1);
        assert_eq!(graph.num_outputs(), 1);

        graph.add_vertex(VertexBuilder::z()
            .coords(1.0, 0.0)
            .build()
        );

        assert_eq!(graph.num_vertices(), 3);
        assert_eq!(graph.num_inputs(), 1);
        assert_eq!(graph.num_outputs(), 1);
    }

    /// Test Graph::add_edge(...)
    #[test]
    fn can_add_edge() {
        let mut graph = Graph::new(1);
        graph.add_wire(1);

        assert_eq!(graph.num_edges(), 1);
        assert_eq!(graph.num_inputs(), 1);
        assert_eq!(graph.num_outputs(), 1);

        graph.add_edge(NodeIndex::new(0), NodeIndex::new(1));
        assert_eq!(graph.num_edges(), 2);
        assert_eq!(graph.num_inputs(), 1);
        assert_eq!(graph.num_outputs(), 1);
    }

    /// Test Graph::input(...)
    #[test]
    fn input_returns_some_if_exists() {
        let mut graph = Graph::new(3);
        graph.add_input(2);
        assert!(graph.input(2).is_some());
    }

    #[test]
    fn input_returns_none_if_does_not_exist() {
        let graph = Graph::new(3);
        assert!(graph.input(2).is_none());
    }

    #[test]
    fn input_returns_none_if_out_of_bounds() {
        let graph = Graph::new(3);
        assert!(graph.input(4).is_none());
    }

    /// Test Graph::output(...)
    #[test]
    fn output_returns_some_if_exists() {
        let mut graph = Graph::new(3);
        graph.add_output(2);
        assert!(graph.output(2).is_some());
    }

    #[test]
    fn output_returns_none_if_does_not_exist() {
        let graph = Graph::new(3);
        assert!(graph.output(2).is_none());
    }

    #[test]
    fn output_returns_none_if_out_of_bounds() {
        let graph = Graph::new(3);
        assert!(graph.output(4).is_none());
    }

    /// Test Graph::is_valid_subgraph()
    #[test]
    fn invalid_subgraph_when_empty() {
        let mut graph = Graph::new(2);
        graph.add_wires(0..2);

        assert!(!graph.is_composable());
    }

    #[test]
    fn valid_subgraph_is_true() {
        let mut graph = Graph::new(2);

        let v1 = graph.add_vertex(VertexBuilder::z()
            .coords(1.0, 1.0)
            .build()
        );

        let v2 = graph.add_vertex(VertexBuilder::x()
            .coords(1.0, 2.0)
            .build()
        );

        graph.add_edge(v1, v2);

        assert!(graph.is_composable());
    }

    #[test]
    fn invalid_subgraph_when_vertex_positions_are_none() {
        let mut graph = Graph::new(2);
        let v1 = graph.add_vertex(VertexBuilder::z().build());
        let v2 = graph.add_vertex(VertexBuilder::x().build());
        graph.add_edge(v1, v2);
        assert!(!graph.is_composable());
    }

    #[test]
    fn num_vertices_of_type_boundary_equal_to_num_inputs_and_outputs() {}
}
