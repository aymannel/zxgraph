use crate::graph::Pauli;
pub(crate) use crate::graph::{BaseGraph, PauliGraph, Vertex};

impl PauliGraph for BaseGraph {
    fn z_pauli(qubit: usize) -> BaseGraph {
        BaseGraph::empty(qubit + 1).with_vertex(Vertex::z_pauli()
            .with_qubit(qubit)
            .with_y(qubit as f64)
            .with_x(1.0)
        )
    }

    fn x_pauli(qubit: usize) -> BaseGraph {
        BaseGraph::empty(qubit + 1).with_vertex(Vertex::x_pauli()
            .with_qubit(qubit)
            .with_y(qubit as f64)
            .with_x(1.0)
        )
    }

    fn y_pauli(qubit: usize) -> BaseGraph {
        BaseGraph::empty(qubit + 1).with_vertex(Vertex::y_pauli()
            .with_qubit(qubit)
            .with_y(qubit as f64)
            .with_x(1.0)
        )
    }
}
