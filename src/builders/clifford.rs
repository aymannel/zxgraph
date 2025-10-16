use crate::builders::{Clifford, GraphBuilder};
use crate::graph::{BaseGraph, VertexBuilder};
use std::cmp::max;

impl Clifford for GraphBuilder {
    /// Builder: Generates a BaseGraph instance of a CX gate
    fn cx(control: usize, target: usize) -> BaseGraph {
        assert_ne!(control, target);
        let mut graph = BaseGraph::new();
        graph.add_wires_along_qubits(0..max(control, target) + 1);

        let z = graph.add_vertex_to_wire(VertexBuilder::z()
            .qubit(control)
            .qubit_coords()
            .build()
        );

        let x = graph.add_vertex_to_wire(VertexBuilder::x()
            .qubit(target)
            .qubit_coords()
            .build()
        );

        graph.add_edge(z, x);
        graph
    }

    /// Builder: Generates a BaseGraph instance of a CZ gate
    fn cz(control: usize, target: usize) -> BaseGraph {
        assert_ne!(control, target);
        let mut graph = BaseGraph::new();
        graph.add_wires_along_qubits(0..max(control, target) + 1);

        let z1 = graph.add_vertex_to_wire(VertexBuilder::z()
            .qubit(control)
            .qubit_coords()
            .build()
        );

        let z2 = graph.add_vertex_to_wire(VertexBuilder::z()
            .qubit(target)
            .qubit_coords()
            .build()
        );

        graph.add_edge(z1, z2);
        graph
    }

    /// Builder: Generates a BaseGraph instance of a z plus gate
    fn z_plus(qubit: usize) -> BaseGraph {
        let mut graph = BaseGraph::new();
        graph.add_wires_along_qubits(0..qubit + 1);
        graph.add_vertex_to_wire(VertexBuilder::z_plus()
            .qubit(qubit)
            .qubit_coords()
            .build()
        );
        graph
    }

    /// Builder: Generates a BaseGraph instance of a z minus gate
    fn z_minus(qubit: usize) -> BaseGraph {
        let mut graph = BaseGraph::new();
        graph.add_wires_along_qubits(0..qubit + 1);
        graph.add_vertex_to_wire(VertexBuilder::z_minus()
            .qubit(qubit)
            .qubit_coords()
            .build()
        );
        graph
    }

    /// Builder: Generates a BaseGraph instance of a x plus gate
    fn x_plus(qubit: usize) -> BaseGraph {
        let mut graph = BaseGraph::new();
        graph.add_wires_along_qubits(0..qubit + 1);
        graph.add_vertex_to_wire(VertexBuilder::x_plus()
            .qubit(qubit)
            .qubit_coords()
            .build()
        );
        graph
    }

    /// Builder: Generates a BaseGraph instance of a x minus gate
    fn x_minus(qubit: usize) -> BaseGraph {
        let mut graph = BaseGraph::new();
        graph.add_wires_along_qubits(0..qubit + 1);
        graph.add_vertex_to_wire(VertexBuilder::x_minus()
            .qubit(qubit)
            .qubit_coords()
            .build()
        );
        graph
    }

    /// Builder: Generates a BaseGraph instance of a y plus gate
    fn y_plus(qubit: usize) -> BaseGraph {
        let mut graph = BaseGraph::new();
        graph.add_wires_along_qubits(0..qubit + 1);
        graph.add_vertex_to_wire(VertexBuilder::y_plus()
            .qubit(qubit)
            .qubit_coords()
            .build()
        );
        graph
    }

    /// Builder: Generates a BaseGraph instance of a y minus gate
    fn y_minus(qubit: usize) -> BaseGraph {
        let mut graph = BaseGraph::new();
        graph.add_wires_along_qubits(0..qubit + 1);
        graph.add_vertex_to_wire(VertexBuilder::y_minus()
            .qubit(qubit)
            .qubit_coords()
            .build()
        );
        graph
    }
}