use crate::graph::phase::Phase;
use crate::graph::{BaseGraph, Clifford, Pauli, Vertex};
use std::cmp::max;

impl Clifford for BaseGraph {
    fn cx(control_qubit: usize, target_qubit: usize) -> BaseGraph {
        assert_ne!(control_qubit, target_qubit);
        let mut graph = BaseGraph::new(max(control_qubit, target_qubit) + 1);

        let control = graph.add_vertex_on_wire(Vertex::z()
            .with_qubit(control_qubit as f64)
            .with_row(1.0));

        let target = graph.add_vertex_on_wire(Vertex::x()
            .with_qubit(target_qubit as f64)
            .with_row(1.0));

        graph.add_edge(control, target);
        graph
    }

    fn cz(control_qubit: usize, target_qubit: usize) -> BaseGraph {
        assert_ne!(control_qubit, target_qubit);
        let mut graph = BaseGraph::new(max(control_qubit, target_qubit) + 1);

        let control = graph.add_vertex_on_wire(Vertex::z()
            .with_qubit(control_qubit as f64)
            .with_row(1.0));

        let target = graph.add_vertex_on_wire(Vertex::z()
            .with_qubit(target_qubit as f64)
            .with_row(1.0));

        graph.add_edge(control, target);
        graph
    }

    fn z_plus(qubit: usize) -> BaseGraph {
        BaseGraph::new(qubit + 1).with_vertex(Vertex::z()
            .with_phase(Phase::from(0.5))
            .with_qubit(qubit as f64)
            .with_row(1.0)
        )
    }

    fn z_minus(qubit: usize) -> BaseGraph {
        BaseGraph::new(qubit + 1).with_vertex(Vertex::z()
            .with_phase(Phase::from(-0.5))
            .with_qubit(qubit as f64)
            .with_row(1.0)
        )
    }

    fn x_plus(qubit: usize) -> BaseGraph {
        BaseGraph::new(qubit + 1).with_vertex(Vertex::x()
            .with_phase(Phase::from(0.5))
            .with_qubit(qubit as f64)
            .with_row(1.0)
        )
    }

    fn x_minus(qubit: usize) -> BaseGraph {
        BaseGraph::new(qubit + 1).with_vertex(Vertex::x()
            .with_phase(Phase::from(-0.5))
            .with_qubit(qubit as f64)
            .with_row(1.0)
        )
    }

    fn y_plus(qubit: usize) -> BaseGraph {
        BaseGraph::new(qubit + 1).with_vertex(Vertex::y()
            .with_phase(Phase::from(0.5))
            .with_qubit(qubit as f64)
            .with_row(1.0)
        )
    }

    fn y_minus(qubit: usize) -> BaseGraph {
        BaseGraph::new(qubit + 1).with_vertex(Vertex::y()
            .with_phase(Phase::from(-0.5))
            .with_qubit(qubit as f64)
            .with_row(1.0)
        )
    }
}