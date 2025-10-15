use crate::graph::phase::Phase;
use crate::graph::{Bases, Boundary, EdgeType, Pauli, Vertex};
use petgraph::prelude::{NodeIndex, StableUnGraph};
use petgraph::stable_graph::{EdgeReferences, NodeReferences};
use petgraph::visit::{IntoEdgeReferences, IntoNodeReferences};

#[derive(Debug, Clone)]
pub struct BaseGraph {
    graph: StableUnGraph<Vertex, EdgeType>,
    inputs: Vec<NodeIndex>,
    outputs: Vec<NodeIndex>,
}

impl BaseGraph {
    /// Creates an empty graph
    pub fn new() -> Self {
        BaseGraph {
            graph: StableUnGraph::with_capacity(0, 0),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    /// Creates an empty graph with the wires specified
    pub fn empty(num_qubits: usize) -> Self {
        assert!(num_qubits > 0, "Number of qubits must be greater than zero");
        BaseGraph::new().with_wires(0..num_qubits)
    }

    /// Builder: Adds a single vertex on the specified qubit
    pub fn with_vertex(mut self, vertex: Vertex) -> Self {
        self.add_vertex_to_wire(vertex);
        self
    }

    /// Builder: Adds a single wire along the specified qubit
    pub fn with_wire(mut self, qubit: usize) -> Self {
        // todo - Do nothing if already exists. See StableGraph::contains_edge()
        let input = self.add_input(qubit);
        let output = self.add_output(qubit);
        self.add_edge(input, output);
        self
    }

    /// Builder: Adds any number of wires along the qubits specified by some iterator
    pub fn with_wires(mut self, qubits: impl IntoIterator<Item = usize>) -> Self {
        for qubit in qubits {
            self = self.with_wire(qubit);
        }
        self
    }

    /// Adds new input boundary node
    pub fn add_input(&mut self, qubit: usize) -> NodeIndex {
        let node = self.graph.add_node(Vertex::b()
            .with_qubit(qubit)
            .with_y(qubit as f64)
            .with_x(0.0)
        );

        self.inputs.push(node);
        node
    }

    // todo - check depth / perhaps create Layout class?
    /// Adds new output boundary
    pub fn add_output(&mut self, qubit: usize) -> NodeIndex {
        let vertex = self.graph.add_node(Vertex::b()
            .with_qubit(qubit)
            .with_y(qubit as f64)
            .with_x(2.0));
        self.outputs.push(vertex);
        vertex
    }

    // todo - improve me!
    /// Returns maximum qubit
    pub fn max_qubit(&self) -> usize {
        self.inputs().iter()
            .filter_map(|&index| self.graph.node_weight(index))
            .map(|vertex| vertex.qubit)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }

    /// Returns vertex by NodeIndex
    pub fn get_vertex(&self, index: NodeIndex) -> Option<&Vertex> {
        self.graph.node_weight(index)
    }

    /// Returns all enumerated vertices in graph
    pub fn enumerate_vertices(&self) -> NodeReferences<'_, Vertex> {
        self.graph.node_references()
    }

    /// Returns all enumerated edges in graph
    pub fn enumerate_edges(&self) -> EdgeReferences<'_, EdgeType> {
        self.graph.edge_references()
    }
    /// Returns total number of vertices
    pub fn num_vertices(&self) -> usize {
        self.graph.node_count()
    }

    /// Returns total number of edges
    pub fn num_edges(&self) -> usize {
        self.graph.edge_count()
    }

    /// Returns all vertices in graph
    pub fn vertices(&self) -> impl Iterator<Item=&Vertex> {
        self.graph.node_weights()
    }

    /// Returns all edges in graph
    pub fn edges(&self) -> impl Iterator<Item=&EdgeType> {
        self.graph.edge_weights()
    }

    /// Returns indices of inputs
    pub fn inputs(&self) -> &[NodeIndex] {
        &self.inputs
    }

    /// Returns indices of outputs
    pub fn outputs(&self) -> &[NodeIndex] {
        &self.outputs
    }

    /// Returns number of inputs
    pub fn num_inputs(&self) -> usize {
        self.inputs.len()
    }

    /// Returns number of outputs
    pub fn num_outputs(&self) -> usize {
        self.outputs.len()
    }

    /// Adds new vertex
    pub fn add_vertex(&mut self, vertex: Vertex) -> NodeIndex {
        self.graph.add_node(vertex)
    }

    /// Adds new z vertex
    pub fn add_z(&mut self, qubit: usize, x: f64, phase: Phase) -> NodeIndex {
        self.add_vertex(Vertex::z()
            .with_phase(phase)
            .with_qubit(qubit)
            .with_y(qubit as f64)
            .with_x(x)
        )
    }

    /// Adds new z zero state
    pub fn add_z_zero(&mut self, qubit: usize, x: f64) -> NodeIndex {
        self.add_vertex(Vertex::z()
            .with_qubit(qubit)
            .with_y(qubit as f64)
            .with_x(x)
        )
    }

    /// Adds new z one state
    pub fn add_z_one(&mut self, qubit: usize, x: f64) -> NodeIndex {
        self.add_vertex(Vertex::z_pauli()
            .with_qubit(qubit)
            .with_y(qubit as f64)
            .with_x(x)
        )
    }

    /// Adds new x vertex
    pub fn add_x(&mut self, qubit: usize, x: f64, phase: Phase) -> NodeIndex {
        self.add_vertex(Vertex::x()
            .with_phase(phase)
            .with_qubit(qubit)
            .with_y(qubit as f64)
            .with_x(x)
        )
    }

    /// Adds new x zero state
    pub fn add_x_zero(&mut self, qubit: usize, x: f64) -> NodeIndex {
        self.add_vertex(Vertex::x()
            .with_qubit(qubit)
            .with_y(qubit as f64)
            .with_x(x)
        )
    }

    /// Adds new x one state
    pub fn add_x_one(&mut self, qubit: usize, x: f64) -> NodeIndex {
        self.add_vertex(Vertex::x_pauli()
            .with_qubit(qubit)
            .with_y(qubit as f64)
            .with_x(x)
        )
    }

    /// Adds new y vertex
    pub fn add_y(&mut self, qubit: usize, x: f64, phase: Phase) -> NodeIndex {
        self.add_vertex(Vertex::y()
            .with_phase(phase)
            .with_qubit(qubit)
            .with_y(qubit as f64)
            .with_x(x)
        )
    }

    /// Adds new y zero state
    pub fn add_y_zero(&mut self, qubit: usize, x: f64) -> NodeIndex {
        self.add_vertex(Vertex::y()
            .with_qubit(qubit)
            .with_y(qubit as f64)
            .with_x(x)
        )
    }

    /// Adds new y one state
    pub fn add_y_one(&mut self, qubit: usize, x: f64) -> NodeIndex {
        self.add_vertex(Vertex::y_pauli()
            .with_qubit(qubit)
            .with_y(qubit as f64)
            .with_x(x)
        )
    }

    /// Adds new edge between two vertices
    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        self.add_edge_of_type(source, target, EdgeType::Simple);
    }

    /// Adds new edge with specified type between two vertices
    pub fn add_edge_of_type(&mut self, source: NodeIndex, target: NodeIndex, edge_type: EdgeType) {
        self.graph.add_edge(source, target, edge_type);
    }

    /// Removes vertex with specified index
    pub fn remove_vertex(&mut self, index: NodeIndex) {
        self.graph.remove_node(index);
    }

    /// Removes edge between two vertices
    pub fn remove_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        self.graph.remove_edge(self.graph
            .find_edge(source, target)
            .expect("No edge found.")
        );
    }

    /// Removes edge between input and output vertices along specified qubit
    pub fn remove_wire(&mut self, qubit: usize) {
        self.remove_edge(
            self.inputs()[qubit],
            self.outputs()[qubit]
        );
    }

    /// Inserts vertex on the edge connecting and input along specified qubit
    pub fn add_vertex_to_wire(&mut self, vertex: Vertex) -> NodeIndex {
        let qubit = vertex.qubit;
        let index = self.add_vertex(vertex);
        self.add_edge(self.inputs()[qubit], index);
        self.add_edge(self.outputs()[qubit], index);
        self.remove_wire(qubit);
        index
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::VertexType;
    use super::*;

    #[test]
    #[should_panic(expected = "Cannot create empty BaseGraph")]
    fn cannot_create_empty_base_graph() {
        BaseGraph::empty(0);
    }

    #[test]
    fn can_create_base_graph_with_capacity() {
        let graph = BaseGraph::empty(3);
        assert_eq!(graph.num_vertices(), 6);
        assert_eq!(graph.num_edges(), 3);
        assert_eq!(graph.num_inputs(), 3);
        assert_eq!(graph.num_outputs(), 3);
    }

    #[test]
    fn can_add_vertex() {
        let mut graph = BaseGraph::empty(1);
        assert_eq!(graph.num_vertices(), 2);
        assert_eq!(graph.num_inputs(), 1);
        assert_eq!(graph.num_outputs(), 1);

        graph.add_vertex(Vertex::new()
            .with_type(VertexType::Z)
            .with_qubit(0)
            .with_x(1.0)
            .with_y(0.0)
        );

        assert_eq!(graph.num_vertices(), 3);
        assert_eq!(graph.num_inputs(), 1);
        assert_eq!(graph.num_outputs(), 1);
    }

    #[test]
    fn can_add_edge() {
        let mut graph = BaseGraph::empty(1);
        assert_eq!(graph.num_edges(), 1);
        assert_eq!(graph.num_inputs(), 1);
        assert_eq!(graph.num_outputs(), 1);

        graph.add_edge(NodeIndex::new(0), NodeIndex::new(1));
        assert_eq!(graph.num_edges(), 2);
        assert_eq!(graph.num_inputs(), 1);
        assert_eq!(graph.num_outputs(), 1);
    }
}
