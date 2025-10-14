use crate::graph::{BaseGraph, Gadget};
use petgraph::prelude::NodeIndex;

impl Gadget for BaseGraph {
    fn gadget(pauli_string: &str, phase: f64) -> BaseGraph {
        let vertical_padding = 0.8;
        let horizontal_padding = 1.8;
        let mut graph = BaseGraph::new(pauli_string.len());

        let hub = graph.add_z(graph.max_qubit() + vertical_padding, horizontal_padding, phase);
        for (qubit, pauli) in pauli_string.chars().enumerate() {
            let optional_index: Option<NodeIndex> = match pauli.to_ascii_lowercase() {
                'z' => Some(graph.add_z_zero(qubit as f64, 1.0)),
                'x' => Some(graph.add_x_zero(qubit as f64, 1.0)),
                'y' => Some(graph.add_y_zero(qubit as f64, 1.0)),
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