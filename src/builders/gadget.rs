use crate::builders::{Gadget, GraphBuilder};
use crate::graph::phase::Phase;
use crate::graph::{BaseGraph, VertexBuilder};

impl Gadget for GraphBuilder {
    /// Builder: Generates a BaseGraph instance of some Pauli Gadget or Phase Gadget
    fn gadget(pauli_string: &str, phase: Phase) -> BaseGraph {
        let mut graph = BaseGraph::new();
        graph.add_wires_along_qubits(0..pauli_string.len());
        let hub = graph.add_vertex(VertexBuilder::z()
            .phase(phase)
            .qubit(0)
            .y_pos(graph.max_qubit() as f64 + 0.8)
            .x_pos(1.8)
            .build()
        );

        for (qubit, pauli) in pauli_string.chars().enumerate() {
            let opt_builder: Option<VertexBuilder> = match pauli.to_ascii_lowercase() {
                'z' => Some(VertexBuilder::z()),
                'x' => Some(VertexBuilder::x()),
                'y' => Some(VertexBuilder::y()),
                _ => {None},
            };

            if let Some(builder) = opt_builder {
                let index = graph.add_vertex_to_wire(builder
                    .qubit(qubit)
                    .qubit_coords()
                    .build()
                );
                graph.add_edge(index, hub);
            }
        }
        graph
    }
}