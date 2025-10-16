use crate::builders::{GraphBuilder, Pauli};
use crate::graph::{BaseGraph, VertexBuilder};

impl Pauli for GraphBuilder {
    /// Builder: Generates a BaseGraph instance of a Pauli Z gate
    fn z_pauli(qubit: usize) -> BaseGraph {
        let mut graph = BaseGraph::new();
        graph.add_wires_along_qubits(0..qubit + 1);
        graph.add_vertex_to_wire(VertexBuilder::z_pauli()
                .qubit(qubit)
                .qubit_coords()
                .build()
            );
        graph
    }

    /// Builder: Generates a BaseGraph instance of a Pauli X gate
    fn x_pauli(qubit: usize) -> BaseGraph {
        let mut graph = BaseGraph::new();
        graph.add_wires_along_qubits(0..qubit + 1);
        graph.add_vertex_to_wire(VertexBuilder::x_pauli()
                .qubit(qubit)
                .qubit_coords()
                .build()
            );
        graph
    }

    /// Builder: Generates a BaseGraph instance of a Pauli Y gate
    fn y_pauli(qubit: usize) -> BaseGraph {
        let mut graph = BaseGraph::new();
        graph.add_wires_along_qubits(0..qubit + 1);
        graph.add_vertex_to_wire(VertexBuilder::y_pauli()
            .qubit(qubit)
            .qubit_coords()
            .build()
        );
        graph
    }
}