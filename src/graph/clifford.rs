use crate::graph::{BaseGraph, Bases, Clifford, CliffordGraph, Vertex};
use std::cmp::max;

impl CliffordGraph for BaseGraph {
    fn cx(control: usize, target: usize) -> BaseGraph {
        assert_ne!(control, target);
        let mut graph = BaseGraph::empty(max(control, target) + 1);

        let z = graph.add_vertex_to_wire(Vertex::z()
            .with_qubit(control)
            .with_y(control as f64)
            .with_x(1.0)
        );

        let x = graph.add_vertex_to_wire(Vertex::x()
            .with_qubit(target)
            .with_y(target as f64)
            .with_x(1.0)
        );

        graph.add_edge(z, x);
        graph
    }

    fn cz(control: usize, target: usize) -> BaseGraph {
        assert_ne!(control, target);
        let mut graph = BaseGraph::empty(max(control, target) + 1);

        let z1 = graph.add_vertex_to_wire(Vertex::z()
            .with_qubit(control)
            .with_y(control as f64)
            .with_x(1.0)
        );

        let z2 = graph.add_vertex_to_wire(Vertex::z()
            .with_qubit(target)
            .with_y(target as f64)
            .with_x(1.0)
        );

        graph.add_edge(z1, z2);
        graph
    }

    fn z_plus(qubit: usize) -> BaseGraph {
        BaseGraph::empty(qubit + 1).with_vertex(Vertex::z_plus()
            .with_qubit(qubit)
            .with_y(qubit as f64)
            .with_x(1.0)
        )
    }

    fn z_minus(qubit: usize) -> BaseGraph {
        BaseGraph::empty(qubit + 1).with_vertex(Vertex::z_minus()
            .with_qubit(qubit)
            .with_y(qubit as f64)
            .with_x(1.0)
        )
    }

    fn x_plus(qubit: usize) -> BaseGraph {
        BaseGraph::empty(qubit + 1).with_vertex(Vertex::x_plus()
            .with_qubit(qubit)
            .with_y(qubit as f64)
            .with_x(1.0)
        )
    }

    fn x_minus(qubit: usize) -> BaseGraph {
        BaseGraph::empty(qubit + 1).with_vertex(Vertex::x_minus()
            .with_qubit(qubit)
            .with_y(qubit as f64)
            .with_x(1.0)
        )
    }

    fn y_plus(qubit: usize) -> BaseGraph {
        BaseGraph::empty(qubit + 1).with_vertex(Vertex::y_plus()
            .with_qubit(qubit)
            .with_y(qubit as f64)
            .with_x(1.0)
        )
    }

    fn y_minus(qubit: usize) -> BaseGraph {
        BaseGraph::empty(qubit + 1).with_vertex(Vertex::y_minus()
            .with_qubit(qubit)
            .with_y(qubit as f64)
            .with_x(1.0)
        )
    }
}