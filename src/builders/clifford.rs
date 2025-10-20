use crate::builders::{Clifford, GraphBuilder};
use crate::graph::{BaseGraph, VertexBuilder};
use std::cmp::max;

impl Clifford for GraphBuilder {
    /// Builder: Generates a BaseGraph instance of a CX gate
    fn cx(control: usize, target: usize) -> BaseGraph {
        assert_ne!(control, target);
        let capacity = max(control, target) + 1;
        let mut graph = BaseGraph::new(capacity);

        let z = graph.add_vertex(VertexBuilder::z()
            .qubit(control)
            .qubit_coords()
            .build()
        );

        let x = graph.add_vertex(VertexBuilder::x()
            .qubit(target)
            .qubit_coords()
            .build()
        );

        let control_input = graph.add_input(control);
        let control_output = graph.add_output(control);
        graph.add_edge(control_input, z);
        graph.add_edge(control_output, z);

        let target_input = graph.add_input(target);
        let target_output = graph.add_output(target);
        graph.add_edge(target_input, x);
        graph.add_edge(target_output, x);

        graph.add_wires_excluding(0..capacity, [target, control]);
        graph.add_edge(z, x);
        graph
    }

    /// Builder: Generates a BaseGraph instance of a CZ gate
    fn cz(control: usize, target: usize) -> BaseGraph {
        assert_ne!(control, target);
        let capacity = max(control, target) + 1;
        let mut graph = BaseGraph::new(capacity);

        let z1 = graph.add_vertex(VertexBuilder::z()
            .qubit(control)
            .qubit_coords()
            .build()
        );

        let z2 = graph.add_vertex(VertexBuilder::z()
            .qubit(target)
            .qubit_coords()
            .build()
        );

        let control_input = graph.add_input(control);
        let control_output = graph.add_output(control);
        graph.add_edge(control_input, z1);
        graph.add_edge(control_output, z1);

        let target_input = graph.add_input(target);
        let target_output = graph.add_output(target);
        graph.add_edge(target_input, z2);
        graph.add_edge(target_output, z2);

        graph.add_wires_excluding(0..capacity, [target, control]);
        graph.add_edge(z1, z2);
        graph
    }

    /// Builder: Generates a BaseGraph instance of a z plus gate
    fn z_plus(qubit: usize) -> BaseGraph {
        let capacity = qubit + 1;
        let mut graph = BaseGraph::new(capacity);
        graph.add_wires_excluding(0..capacity, [qubit]);

        let vertex = graph.add_vertex(VertexBuilder::z_plus()
            .qubit(qubit)
            .qubit_coords()
            .build()
        );

        let input = graph.input(qubit).unwrap();
        let output = graph.output(qubit).unwrap();
        graph.add_edge(input, vertex);
        graph.add_edge(output, vertex);
        graph
    }

    /// Builder: Generates a BaseGraph instance of a z minus gate
    fn z_minus(qubit: usize) -> BaseGraph {
        let capacity = qubit + 1;
        let mut graph = BaseGraph::new(capacity);
        graph.add_wires_excluding(0..capacity, [qubit]);

        let vertex = graph.add_vertex(VertexBuilder::z_minus()
            .qubit(qubit)
            .qubit_coords()
            .build()
        );

        let input = graph.add_input(qubit);
        let output = graph.add_output(qubit);
        graph.add_edge(input, vertex);
        graph.add_edge(output, vertex);
        graph
    }

    /// Builder: Generates a BaseGraph instance of a x plus gate
    fn x_plus(qubit: usize) -> BaseGraph {
        let capacity = qubit + 1;
        let mut graph = BaseGraph::new(capacity);
        graph.add_wires_excluding(0..capacity, [qubit]);

        let vertex = graph.add_vertex(VertexBuilder::x_plus()
            .qubit(qubit)
            .qubit_coords()
            .build()
        );

        let input = graph.add_input(qubit);
        let output = graph.add_output(qubit);
        graph.add_edge(input, vertex);
        graph.add_edge(output, vertex);
        graph
    }

    /// Builder: Generates a BaseGraph instance of a x minus gate
    fn x_minus(qubit: usize) -> BaseGraph {
        let capacity = qubit + 1;
        let mut graph = BaseGraph::new(capacity);
        graph.add_wires_excluding(0..capacity, [qubit]);

        let vertex = graph.add_vertex(VertexBuilder::x_minus()
            .qubit(qubit)
            .qubit_coords()
            .build()
        );

        let input = graph.add_input(qubit);
        let output = graph.add_output(qubit);
        graph.add_edge(input, vertex);
        graph.add_edge(output, vertex);
        graph
    }

    /// Builder: Generates a BaseGraph instance of a y plus gate
    fn y_plus(qubit: usize) -> BaseGraph {
        let capacity = qubit + 1;
        let mut graph = BaseGraph::new(capacity);
        graph.add_wires_excluding(0..capacity, [qubit]);

        let vertex = graph.add_vertex(VertexBuilder::y_plus()
            .qubit(qubit)
            .qubit_coords()
            .build()
        );

        let input = graph.add_input(qubit);
        let output = graph.add_output(qubit);
        graph.add_edge(input, vertex);
        graph.add_edge(output, vertex);
        graph
    }

    /// Builder: Generates a BaseGraph instance of a y minus gate
    fn y_minus(qubit: usize) -> BaseGraph {
        let capacity = qubit + 1;
        let mut graph = BaseGraph::new(capacity);
        graph.add_wires_excluding(0..capacity, [qubit]);

        let vertex = graph.add_vertex(VertexBuilder::y_minus()
            .qubit(qubit)
            .qubit_coords()
            .build()
        );

        let input = graph.add_input(qubit);
        let output = graph.add_output(qubit);
        graph.add_edge(input, vertex);
        graph.add_edge(output, vertex);
        graph
    }
}