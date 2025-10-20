use crate::builders::{GraphBuilder, Pauli};
use crate::graph::{Graph, VertexBuilder};

impl Pauli for GraphBuilder {
    /// Builder: Generates a BaseGraph instance of a Pauli Z gate
    fn pauli_z(qubit: usize) -> Graph {
        let capacity = qubit + 1;
        let mut graph = Graph::new(capacity);

        graph.add_wires_excluding(0..capacity, [qubit]);
        graph.add_vertex_on_wire(VertexBuilder::z_pauli()
            .qubit(qubit)
            .qubit_coords()
            .build()
        );

        graph
    }

    /// Builder: Generates a BaseGraph instance of a Pauli X gate
    fn pauli_x(qubit: usize) -> Graph {
        let capacity = qubit + 1;
        let mut graph = Graph::new(capacity);

        graph.add_wires_excluding(0..capacity, [qubit]);
        graph.add_vertex_on_wire(VertexBuilder::x_pauli()
            .qubit(qubit)
            .qubit_coords()
            .build()
        );

        graph
    }

    /// Builder: Generates a BaseGraph instance of a Pauli Y gate
    fn pauli_y(qubit: usize) -> Graph {
        let capacity = qubit + 1;
        let mut graph = Graph::new(capacity);

        graph.add_wires_excluding(0..capacity, [qubit]);
        graph.add_vertex_on_wire(VertexBuilder::y_pauli()
            .qubit(qubit)
            .qubit_coords()
            .build()
        );

        graph
    }
}


#[cfg(test)]
mod tests {
    use crate::builders::{GraphBuilder, Pauli};

    #[test]
    fn pauli_z() {
        let pauli_z = GraphBuilder::pauli_z(0);
        assert_eq!(pauli_z.capacity(), 1);
        assert_eq!(pauli_z.num_inputs(), 1);
        assert_eq!(pauli_z.num_outputs(), 1);
        assert_eq!(pauli_z.num_vertices(), 3);
        assert_eq!(pauli_z.num_edges(), 2);
    }

    #[test]
    fn pauli_x() {
        let pauli_x = GraphBuilder::pauli_x(0);
        assert_eq!(pauli_x.capacity(), 1);
        assert_eq!(pauli_x.num_inputs(), 1);
        assert_eq!(pauli_x.num_outputs(), 1);
        assert_eq!(pauli_x.num_vertices(), 3);
        assert_eq!(pauli_x.num_edges(), 2);
    }

    #[test]
    fn pauli_y() {
        let pauli_y = GraphBuilder::pauli_y(0);
        assert_eq!(pauli_y.capacity(), 1);
        assert_eq!(pauli_y.num_inputs(), 1);
        assert_eq!(pauli_y.num_outputs(), 1);
        assert_eq!(pauli_y.num_vertices(), 3);
        assert_eq!(pauli_y.num_edges(), 2);
    }
}