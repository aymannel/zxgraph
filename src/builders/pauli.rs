use crate::builders::{GraphBuilder, Pauli};
use crate::graph::{BaseGraph, VertexBuilder};

impl Pauli for GraphBuilder {
    /// Builder: Generates a BaseGraph instance of a Pauli Z gate
    fn z_pauli(qubit: usize) -> BaseGraph {
        let capacity = qubit + 1;
        let mut graph = BaseGraph::new(capacity);
        graph.add_wires_excluding(0..capacity, [qubit]);

        let vertex = graph.add_vertex(VertexBuilder::z_pauli()
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

    /// Builder: Generates a BaseGraph instance of a Pauli X gate
    fn x_pauli(qubit: usize) -> BaseGraph {
        let capacity = qubit + 1;
        let mut graph = BaseGraph::new(capacity);
        graph.add_wires_excluding(0..capacity, [qubit]);

        let vertex = graph.add_vertex(VertexBuilder::x_pauli()
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

    /// Builder: Generates a BaseGraph instance of a Pauli Y gate
    fn y_pauli(qubit: usize) -> BaseGraph {
        let capacity = qubit + 1;
        let mut graph = BaseGraph::new(capacity);
        graph.add_wires_excluding(0..capacity, [qubit]);

        let vertex = graph.add_vertex(VertexBuilder::y_pauli()
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