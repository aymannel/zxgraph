use crate::graph::phase::Phase;
use crate::graph::{EdgeType, Vertex, VertexType};
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
    /// Creates empty graph with estimated qubit capacity
    pub fn new(num_qubits: usize) -> Self {
        assert!(num_qubits > 0, "Cannot create empty BaseGraph");
        let inputs = Vec::with_capacity(num_qubits);
        let outputs = Vec::with_capacity(num_qubits);
        let graph = StableUnGraph::with_capacity(2 * num_qubits, num_qubits);

        let mut base_graph = BaseGraph {
            graph,
            inputs,
            outputs,
        };

        for qubit in 0..num_qubits {
            let input = base_graph.add_input(qubit as f64);
            let output = base_graph.add_output(qubit as f64);
            base_graph.add_edge(input, output);
        }

        base_graph
    }

    /// Creates BaseGraph with a single vertex on the specified qubit
    pub fn with_vertex(qubit: usize, vertex_type: VertexType, phase: f64) -> BaseGraph {
        let mut graph = BaseGraph::new(qubit + 1);
        let vertex_index = graph.add_vertex(Vertex {
            vertex_type,
            phase: Phase::from(phase),
            qubit: qubit as f64,
            row: 1.0
        });
        graph.add_edge(vertex_index, graph.inputs()[qubit]);
        graph.add_edge(vertex_index, graph.outputs()[qubit]);
        graph.remove_wire(qubit);
        graph
    }

    /// Creates new input boundary node
    pub fn add_input(&mut self, qubit: f64) -> NodeIndex {
        let node = self.graph.add_node(Vertex {
            vertex_type: VertexType::B,
            phase: Phase::from(0.0),
            row: 0.0,
            qubit,
        });
        self.inputs.push(node);
        node
    }

    /// Creates new output boundary
    pub fn add_output(&mut self, qubit: f64) -> NodeIndex {
        let vertex = self.graph.add_node(Vertex {
            vertex_type: VertexType::B,
            phase: Phase::from(0.0),
            row: 2.0,  // check graph depth here
            qubit,
        });
        self.outputs.push(vertex);
        vertex
    }

    /// Returns maximum qubit
    pub fn max_qubit(&self) -> f64 {
        self.inputs().iter()
            .filter_map(|&index| self.graph.node_weight(index))
            .map(|vertex| vertex.qubit)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
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

    /// Creates new vertex
    pub fn add_vertex(&mut self, vertex: Vertex) -> NodeIndex {
        self.graph.add_node(vertex)
    }

    /// Creates new z vertex
    pub fn add_z(&mut self, qubit: f64, row: f64, phase: f64) -> NodeIndex {
        self.add_vertex(Vertex {vertex_type: VertexType::Z, phase: Phase::from(phase), row, qubit})
    }

    /// Creates new z zero state
    pub fn add_z_zero(&mut self, qubit: f64, row: f64) -> NodeIndex {
        self.add_vertex(Vertex {vertex_type: VertexType::Z, phase: Phase::from(0.0), row, qubit})
    }

    /// Creates new z one state
    pub fn add_z_one(&mut self, qubit: f64, row: f64) -> NodeIndex {
        self.add_vertex(Vertex {vertex_type: VertexType::Z, phase: Phase::from(1.0), row, qubit})
    }

    /// Creates new x vertex
    pub fn add_x(&mut self, qubit: f64, row: f64, phase: f64) -> NodeIndex {
        self.add_vertex(Vertex {vertex_type: VertexType::X, phase: Phase::from(phase), row, qubit})
    }

    /// Creates new x zero state
    pub fn add_x_zero(&mut self, qubit: f64, row: f64) -> NodeIndex {
        self.add_vertex(Vertex {vertex_type: VertexType::X, phase: Phase::from(0.0), row, qubit})
    }

    /// Creates new x one state
    pub fn add_x_one(&mut self, qubit: f64, row: f64) -> NodeIndex {
        self.add_vertex(Vertex {vertex_type: VertexType::X, phase: Phase::from(1.0), row, qubit})
    }

    /// Creates new y vertex
    pub fn add_y(&mut self, qubit: f64, row: f64, phase: f64) -> NodeIndex {
        self.add_vertex(Vertex {vertex_type: VertexType::Y, phase: Phase::from(phase), row, qubit})
    }

    /// Creates new y zero state
    pub fn add_y_zero(&mut self, qubit: f64, row: f64) -> NodeIndex {
        self.add_vertex(Vertex {vertex_type: VertexType::Y, phase: Phase::from(0.0), row, qubit})
    }

    /// Creates new y one state
    pub fn add_y_one(&mut self, qubit: f64, row: f64) -> NodeIndex {
        self.add_vertex(Vertex {vertex_type: VertexType::Y, phase: Phase::from(1.0), row, qubit})
    }

    /// Creates new edge between two vertices
    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        self.add_edge_of_type(source, target, EdgeType::Simple);
    }

    /// Creates new edge with specified type between two vertices
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

    /// Enumerates all vertices in graph
    pub fn enumerate_vertices(&self) -> NodeReferences<'_, Vertex> {
        self.graph.node_references()
    }

    /// Enumerates all edges in graph
    pub fn enumerate_edges(&self) -> EdgeReferences<'_, EdgeType> {
        self.graph.edge_references()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Cannot create empty BaseGraph")]
    fn cannot_create_empty_base_graph() {
        BaseGraph::new(0);
    }

    #[test]
    fn can_create_base_graph_with_capacity() {
        let graph = BaseGraph::new(3);
        assert_eq!(graph.num_vertices(), 6);
        assert_eq!(graph.num_edges(), 3);
        assert_eq!(graph.num_inputs(), 3);
        assert_eq!(graph.num_outputs(), 3);
    }

    #[test]
    fn can_add_vertex() {
        let mut graph = BaseGraph::new(1);
        assert_eq!(graph.num_vertices(), 2);
        assert_eq!(graph.num_inputs(), 1);
        assert_eq!(graph.num_outputs(), 1);

        graph.add_vertex(Vertex {
            vertex_type: VertexType::B,
            phase: Phase::from(0.0),
            qubit: 1.0,
            row: 1.0
        });

        assert_eq!(graph.num_vertices(), 3);
        assert_eq!(graph.num_inputs(), 1);
        assert_eq!(graph.num_outputs(), 1);
    }

    #[test]
    fn can_add_edge() {
        let mut graph = BaseGraph::new(1);
        assert_eq!(graph.num_edges(), 1);
        assert_eq!(graph.num_inputs(), 1);
        assert_eq!(graph.num_outputs(), 1);

        graph.add_edge(NodeIndex::new(0), NodeIndex::new(1));
        assert_eq!(graph.num_edges(), 2);
        assert_eq!(graph.num_inputs(), 1);
        assert_eq!(graph.num_outputs(), 1);
    }
}
