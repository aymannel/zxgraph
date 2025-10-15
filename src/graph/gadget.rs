use crate::graph::{BaseGraph, Bases, GadgetGraph, Vertex};
use petgraph::prelude::NodeIndex;
use crate::graph::phase::Phase;

impl GadgetGraph for BaseGraph {
    fn gadget(pauli_string: &str, phase: Phase) -> BaseGraph {
        let mut graph = BaseGraph::empty(pauli_string.len());
        let hub = graph.add_vertex(Vertex::z()
            .with_phase(phase)
            .with_y(graph.max_qubit() as f64 + 0.8)
            .with_x(1.8)
        );
        
        for (qubit, pauli) in pauli_string.chars().enumerate() {
            let optional_index: Option<NodeIndex> = match pauli.to_ascii_lowercase() {
                'z' => Some(graph.add_z_zero(qubit, 1.0)),
                'x' => Some(graph.add_x_zero(qubit, 1.0)),
                'y' => Some(graph.add_y_zero(qubit, 1.0)),
                _ => {None},
            };

            if let Some(index) = optional_index {
                graph.remove_wire(qubit);
                graph.add_edge(index, hub);
                graph.add_edge(index, graph.inputs()[qubit]);
                graph.add_edge(index, graph.outputs()[qubit]);
            }
        }
        graph
    }
}