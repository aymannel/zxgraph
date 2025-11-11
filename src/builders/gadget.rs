use crate::builders::{Gadget, GraphBuilder};
use crate::graph::phase::Phase;
use crate::graph::{Graph, VertexBuilder, VertexType};

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
            let opt_vertex_type: Option<VertexType> = match pauli {
                'i' | 'I' => None,
                'z' | 'Z' => Some(VertexType::Z),
                'x' | 'X' => Some(VertexType::X),
                'y' | 'Y' => Some(VertexType::Y),
                _ => panic!("invalid pauli character")
            };

            if let Some(vertex_type) = opt_vertex_type {
                let vertex = graph.add_unary(qubit, VertexBuilder::new()
                    .vertex_type(vertex_type)
                    .coords(0.0, qubit as f64)
                    .build()
                );
                graph.add_edge(vertex, hub);
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
        assert_eq!(gadget.num_vertices(), 4);
        assert_eq!(gadget.num_edges(), 3);
    }
}
