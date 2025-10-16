use crate::builders::{GraphBuilder, Pauli};
use crate::graph::{BaseGraph, VertexBuilder};

impl Pauli for GraphBuilder {
    /// Builder: Generates a BaseGraph instance of a Pauli Z gate
    fn z_pauli(qubit: usize) -> BaseGraph {
        BaseGraph::empty(qubit + 1)
            .with_vertex(VertexBuilder::z_pauli()
                .qubit(qubit)
                .qubit_coords()
                .build()
            )
    }

    /// Builder: Generates a BaseGraph instance of a Pauli X gate
    fn x_pauli(qubit: usize) -> BaseGraph {
        BaseGraph::empty(qubit + 1)
            .with_vertex(VertexBuilder::x_pauli()
                .qubit(qubit)
                .qubit_coords()
                .build()
            )
    }

    /// Builder: Generates a BaseGraph instance of a Pauli Y gate
    fn y_pauli(qubit: usize) -> BaseGraph {
        BaseGraph::empty(qubit + 1)
            .with_vertex(VertexBuilder::y_pauli()
                .qubit(qubit)
                .qubit_coords()
                .build()
            )
    }
}