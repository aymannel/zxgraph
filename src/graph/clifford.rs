use crate::graph::{BaseGraph, Clifford, VertexType};

impl Clifford for BaseGraph {
    fn cx(control: usize, target: usize) -> BaseGraph {
        todo!()
    }

    fn cz(control: usize, target: usize) -> BaseGraph {
        todo!()
    }

    fn z_plus(qubit: usize) -> BaseGraph {
        BaseGraph::with_vertex(qubit, VertexType::Z, 0.5)
    }

    fn z_minus(qubit: usize) -> BaseGraph {
        BaseGraph::with_vertex(qubit, VertexType::Z, -0.5)
    }

    fn x_plus(qubit: usize) -> BaseGraph {
        BaseGraph::with_vertex(qubit, VertexType::X, 0.5)
    }

    fn x_minus(qubit: usize) -> BaseGraph {
        BaseGraph::with_vertex(qubit, VertexType::X, -0.5)
    }

    fn y_plus(qubit: usize) -> BaseGraph {
        BaseGraph::with_vertex(qubit, VertexType::Y, 0.5)
    }

    fn y_minus(qubit: usize) -> BaseGraph {
        BaseGraph::with_vertex(qubit, VertexType::Y, -0.5)
    }
}