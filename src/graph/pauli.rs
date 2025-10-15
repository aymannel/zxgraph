pub(crate) use crate::graph::{BaseGraph, Pauli, Vertex};

impl Pauli for BaseGraph {
    fn z(qubit: usize) -> BaseGraph {
        BaseGraph::new(qubit + 1).with_vertex(Vertex::z()
            .with_qubit(qubit as f64)
            .with_row(1.0)
        )
    }

    fn x(qubit: usize) -> BaseGraph {
        BaseGraph::new(qubit + 1).with_vertex(Vertex::x()
            .with_qubit(qubit as f64)
            .with_row(1.0)
        )
    }

    fn y(qubit: usize) -> BaseGraph {
        BaseGraph::new(qubit + 1).with_vertex(Vertex::y()
            .with_qubit(qubit as f64)
            .with_row(1.0)
        )
    }
}
