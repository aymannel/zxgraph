use crate::graph::{BaseGraph, Pauli, VertexType};

impl Pauli for BaseGraph {
    fn z(qubit: usize) -> BaseGraph {
        BaseGraph::with_vertex(qubit, VertexType::Z, 1.0)
    }

    fn x(qubit: usize) -> BaseGraph {
        BaseGraph::with_vertex(qubit, VertexType::X, 1.0)
    }

    fn y(qubit: usize) -> BaseGraph {
        BaseGraph::with_vertex(qubit, VertexType::Y, 1.0)
    }
}
