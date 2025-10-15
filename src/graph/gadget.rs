use crate::graph::phase::Phase;
use crate::graph::{BaseGraph, Bases, GadgetGraph, Vertex};

impl GadgetGraph for BaseGraph {
    fn gadget(pauli_string: &str, phase: Phase) -> BaseGraph {
        let mut graph = BaseGraph::empty(pauli_string.len());
        let hub = graph.add_vertex(Vertex::z()
            .with_phase(phase)
            .with_y(graph.max_qubit() as f64 + 0.8)
            .with_x(1.8)
        );

        for (qubit, pauli) in pauli_string.chars().enumerate() {
            let opt_vertex: Option<Vertex> = match pauli.to_ascii_lowercase() {
                'z' => Some(Vertex::z()),
                'x' => Some(Vertex::x()),
                'y' => Some(Vertex::y()),
                _ => {None},
            };

            if let Some(vertex) = opt_vertex {
                let index = graph.add_vertex_to_wire(vertex
                    .with_qubit(qubit)
                    .with_quords()
                );
                graph.add_edge(index, hub);
            }
        }

        graph
    }
}