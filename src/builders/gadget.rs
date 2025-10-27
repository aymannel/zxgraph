use crate::builders::{Gadget, GraphBuilder};
use crate::graph::phase::Phase;
use crate::graph::{Graph, VertexBuilder};

impl Gadget for GraphBuilder {
    /// Builder: Generates a BaseGraph instance of some Pauli Gadget or Phase Gadget
    fn gadget(pauli_string: &str, phase: Phase) -> Graph {
        let pauli_len = pauli_string.len();
        let mut graph = Graph::new(pauli_len);

        let hub = graph.add_vertex(VertexBuilder::z()
            .coords(0.8, pauli_len as f64)
            .phase(phase)
            .build()
        );

        for (qubit, pauli) in pauli_string.chars().enumerate() {
            let opt_builder: Option<VertexBuilder> = match pauli.to_ascii_lowercase() {
                'z' => Some(VertexBuilder::z()),
                'x' => Some(VertexBuilder::x()),
                'y' => Some(VertexBuilder::y()),
                'i' => None,
                _ => panic!("invalid pauli character")
            };

            if let Some(builder) = opt_builder {
                let vertex = graph.add_vertex_on_wire(qubit, builder
                    .coords(0.0, qubit as f64)
                    .build()
                );
                graph.add_edge(vertex, hub);
            } else {
                graph.add_wire(qubit)
            }
        }
        graph
    }
}


#[cfg(test)]
mod tests {
    use crate::builders::{Gadget, GraphBuilder};
    use crate::graph::phase::Phase;

    #[test]
    #[should_panic(expected = "invalid pauli character")]
    fn should_panic_when_invalid_pauli_character() {
        GraphBuilder::gadget("ZXH", Phase::zero());
    }

    #[test]
    fn gadget() {
        let gadget = GraphBuilder::gadget("zxy", Phase::zero());
        assert_eq!(gadget.num_inputs(), 3);
        assert_eq!(gadget.num_outputs(), 3);
        assert_eq!(gadget.num_vertices(), 10);
        assert_eq!(gadget.num_edges(), 9);
    }
}
