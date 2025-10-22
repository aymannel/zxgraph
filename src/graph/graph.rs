use crate::graph::{EdgeType, Vertex, VertexBuilder, VertexType};
use petgraph::prelude::{EdgeIndex, NodeIndex, StableUnGraph};
use petgraph::stable_graph::{EdgeReferences, NodeReferences};
use petgraph::visit::{IntoEdgeReferences, IntoNodeReferences};

#[derive(Debug, Clone)]
pub struct Graph {
    base_graph: StableUnGraph<Vertex, EdgeType>,
    inputs: Vec<Option<NodeIndex>>,
    outputs: Vec<Option<NodeIndex>>,
}

impl Graph {
    /// Creates an empty graph
    pub fn new(capacity: usize) -> Self {
        Graph {
            base_graph: StableUnGraph::with_capacity(2 * capacity, capacity),
            inputs: vec![None; capacity],
            outputs: vec![None; capacity],
        }
    }

    /// Resizes input and output capacity to match target capacity
    pub fn ensure_capacity(&mut self, target_capacity: usize) {
        if self.input_capacity() < target_capacity {
            self.inputs.resize(target_capacity, None);
        }

        if self.output_capacity() < target_capacity {
            self.outputs.resize(target_capacity, None);
        }
    }

    /// Adds new input boundary node
    pub fn add_input(&mut self, qubit: usize) -> NodeIndex {
        self.ensure_capacity(qubit + 1);
        let input = self.base_graph.add_node(VertexBuilder::b()
            .qubit(qubit)
            .qubit_coords()
            .x_pos(-1.0)
            .build()
        );
        self.inputs[qubit] = Some(input);
        input
    }

    /// Adds new output boundary
    pub fn add_output(&mut self, qubit: usize) -> NodeIndex {
        self.ensure_capacity(qubit + 1);
        let output = self.base_graph.add_node(VertexBuilder::b()
            .qubit(qubit)
            .qubit_coords()
            .x_pos(1.0)
            .build()
        );
        self.outputs[qubit] = Some(output);
        output
    }

    /// Adds a single wire along the specified qubit
    pub fn add_wire(&mut self, qubit: usize) {
        self.ensure_capacity(qubit + 1);
        match (self.input(qubit), self.output(qubit)) {
            (Some(input), Some(output)) => {
                if !self.base_graph.contains_edge(input, output) {
                    self.add_edge(input, output)
                }
            }
            (None, None) => {
                let input = self.add_input(qubit);
                let output = self.add_output(qubit);
                self.add_edge(input, output)
            }
            _ => panic!("input and output mismatch at qubit index {qubit}"),
        }
    }

    /// Adds wires along the specified qubits
    pub fn add_wires(&mut self, qubits: impl IntoIterator<Item = usize>) {
        for qubit in qubits {
            self.add_wire(qubit);
        }
    }

    /// Adds wires along the specified qubits excluding specified qubits
    pub fn add_wires_excluding<I, E>(&mut self, qubits: I, excluded: E)
    where
        I: IntoIterator<Item = usize>,
        E: IntoIterator<Item = usize>,
    {
        let excluded: Vec<_> = excluded.into_iter().collect();
        self.add_wires(
            qubits.into_iter().filter(|i| !excluded.contains(i))
        );
    }

    /// Returns true if self is a valid subgraph
    pub fn is_valid_subgraph(&self) -> bool {
        self.is_occupied() && self.vertices_all_positioned()
    }

    /// Returns true if all vertices in graph have positions
    fn vertices_all_positioned(&self) -> bool {
        self.vertices().all(|v| v.y_pos().is_some() && v.x_pos().is_some())
    }

    /// Returns true if not all vertices in graph are boundaries
    fn is_occupied(&self) -> bool {
        self.vertices().filter(|v| v.vertex_type() != VertexType::B).count() > 0
    }

    /// Returns vertex by NodeIndex
    pub fn vertex(&self, index: NodeIndex) -> Option<&Vertex> {
        self.base_graph.node_weight(index)
    }

    /// Returns optional Vertex by NodeIndex
    pub fn edge_index(&self, source: NodeIndex, target: NodeIndex) -> Option<EdgeIndex> {
        self.base_graph.find_edge(source, target)
    }

    /// Returns all enumerated vertices in graph
    pub fn enumerate_vertices(&self) -> NodeReferences<'_, Vertex> {
        self.base_graph.node_references()
    }

    /// Returns all enumerated edges in graph
    pub fn enumerate_edges(&self) -> EdgeReferences<'_, EdgeType> {
        self.base_graph.edge_references()
    }

    /// Returns total number of vertices
    pub fn num_vertices(&self) -> usize {
        self.base_graph.node_count()
    }

    /// Returns total number of edges
    pub fn num_edges(&self) -> usize {
        self.base_graph.edge_count()
    }

    /// Returns all vertices in graph
    pub fn vertices(&self) -> impl Iterator<Item=&Vertex> {
        self.base_graph.node_weights()
    }

    /// Returns all edges in graph
    pub fn edges(&self) -> impl Iterator<Item=&EdgeType> {
        self.base_graph.edge_weights()
    }

    /// Returns indices of inputs
    pub fn inputs(&self) -> &[Option<NodeIndex>] {
        &self.inputs
    }

    /// Returns indices of outputs
    pub fn outputs(&self) -> &[Option<NodeIndex>] {
        &self.outputs
    }

    /// Returns input capacity (distinct from rust vector capacity)
    pub fn input_capacity(&self) -> usize {
        self.inputs.len()
    }

    /// Returns input capacity (distinct from rust vector capacity)
    pub fn output_capacity(&self) -> usize {
        self.outputs.len()
    }

    /// Returns number of inputs present
    pub fn num_inputs(&self) -> usize {
        self.inputs().iter().flatten().count()
    }

    /// Returns number of outputs present
    pub fn num_outputs(&self) -> usize {
        self.outputs().iter().flatten().count()
    }

    /// Adds new vertex
    pub fn add_vertex(&mut self, vertex: Vertex) -> NodeIndex {
        self.ensure_capacity(vertex.qubit() + 1);
        self.base_graph.add_node(vertex)
    }

    /// Adds new vertex and wire along the specified qubit
    pub fn add_vertex_on_wire(&mut self, vertex: Vertex) -> NodeIndex {
        let input = self.add_input(vertex.qubit());
        let output = self.add_output(vertex.qubit());
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

    /// Removes edge between input and output vertices along specified qubit
    pub fn remove_wire(&mut self, qubit: usize) {
        if let (Some(input), Some(output)) = (self.input(qubit), self.output(qubit)) {
            self.remove_edge(input, output);
        }
    }

    /// Returns NodeIndex if input exists at qubit
    pub fn input(&mut self, qubit: usize) -> Option<NodeIndex> {
        assert!(qubit < self.input_capacity(), "qubit index larger than input capacity");
        self.inputs[qubit]
    }

    /// Returns NodeIndex if output exists at qubit
    pub fn output(&mut self, qubit: usize) -> Option<NodeIndex> {
        assert!(qubit < self.output_capacity(), "qubit index larger than output capacity");
        self.outputs[qubit]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_empty_graph() {
        let graph = Graph::new(3);
        assert_eq!(graph.num_edges(), 0);
        assert_eq!(graph.num_vertices(), 0);

        // Check inputs and outputs
        assert_eq!(graph.num_inputs(), 0);
        assert_eq!(graph.num_outputs(), 0);

        // Check capacity
        assert_eq!(graph.input_capacity(), 3);
        assert_eq!(graph.output_capacity(), 3);
    }

    #[test]
    fn can_create_base_graph_with_capacity() {
        // When
        let mut graph = Graph::new(3);
        graph.add_wires(0..3);

        // Then
        assert_eq!(graph.num_vertices(), 6);
        assert_eq!(graph.num_edges(), 3);
        assert_eq!(graph.num_inputs(), 3);
        assert_eq!(graph.num_outputs(), 3);
    }

    #[test]
    fn can_add_vertex() {
        // When
        let mut graph = Graph::new(1);
        graph.add_wire(1);

        // Then
        assert_eq!(graph.num_vertices(), 2);
        assert_eq!(graph.num_inputs(), 1);
        assert_eq!(graph.num_outputs(), 1);

        graph.add_vertex(VertexBuilder::z()
            .qubit(0)
            .x_pos(1.0)
            .y_pos(0.0)
            .build()
        );

        assert_eq!(graph.num_vertices(), 3);
        assert_eq!(graph.num_inputs(), 1);
        assert_eq!(graph.num_outputs(), 1);
    }

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

    #[test]
    fn invalid_subgraph_when_empty() {
        let mut graph = Graph::new(2);
        graph.add_wires(0..2);

        assert!(!graph.is_valid_subgraph());
    }

    #[test]
    fn valid_subgraph_is_true() {
        let mut graph = Graph::new(2);

        let v1 = graph.add_vertex(VertexBuilder::z().qubit(0)
            .x_pos(1.0)
            .y_pos(1.0)
            .build()
        );

        let v2 = graph.add_vertex(VertexBuilder::x().qubit(1)
            .x_pos(1.0)
            .y_pos(2.0)
            .build()
        );

        graph.add_edge(v1, v2);

        assert!(graph.is_valid_subgraph());
    }

    #[test]
    fn invalid_subgraph_when_vertex_positions_are_none() {
        let mut graph = Graph::new(2);
        let v1 = graph.add_vertex(VertexBuilder::z().qubit(0).build());
        let v2 = graph.add_vertex(VertexBuilder::x().qubit(1).build());
        graph.add_edge(v1, v2);

        assert!(!graph.is_valid_subgraph());
    }
}
