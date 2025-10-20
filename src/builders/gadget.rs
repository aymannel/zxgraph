use crate::builders::{Gadget, GraphBuilder};
use crate::graph::phase::Phase;
use crate::graph::{BaseGraph, VertexBuilder};

impl Gadget for GraphBuilder {
    /// Builder: Generates a BaseGraph instance of some Pauli Gadget or Phase Gadget
    fn gadget(pauli_string: &str, phase: Phase) -> BaseGraph {
        let mut graph = BaseGraph::new(pauli_string.len());
        let hub = graph.add_vertex(VertexBuilder::z()
            .phase(phase)
            .qubit(0)
            .y_pos(graph.capacity() as f64)
            .x_pos(0.8)
            .build()
        );

        for (qubit, pauli) in pauli_string.chars().enumerate() {
            let opt_builder: Option<VertexBuilder> = match pauli.to_ascii_lowercase() {
                'z' => Some(VertexBuilder::z()),
                'x' => Some(VertexBuilder::x()),
                'y' => Some(VertexBuilder::y()),
                'i' => {None},
                _ => panic!("invalid pauli character!")
            };

            if let Some(builder) = opt_builder {
                let input = graph.add_input(qubit);
                let output = graph.add_output(qubit);
                let vertex = graph.add_vertex(builder
                    .qubit(qubit)
                    .qubit_coords()
                    .build()
                );

                graph.add_edge(input, vertex);
                graph.add_edge(output, vertex);
                graph.add_edge(vertex, hub);
            } else {
                graph.add_wire(qubit);
            }
        }
        graph
    }
}