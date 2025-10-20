use crate::builders::{GraphBuilder, Pauli};
use crate::graph::{Graph, VertexBuilder};

impl Pauli for GraphBuilder {
    /// Builder: Generates a BaseGraph instance of a Pauli Z gate
    fn z_pauli(qubit: usize) -> Graph {
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
    fn x_pauli(qubit: usize) -> Graph {
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
    fn y_pauli(qubit: usize) -> Graph {
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