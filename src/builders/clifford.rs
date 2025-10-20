use crate::builders::{Clifford, GraphBuilder};
use crate::graph::{Graph, VertexBuilder};
use std::cmp::max;

impl Clifford for GraphBuilder {
    /// Builder: Generates a BaseGraph instance of a CX gate
    fn cx(control: usize, target: usize) -> Graph {
        assert_ne!(control, target);
        let capacity = max(control, target) + 1;
        let mut graph = Graph::new(capacity);

        let z = graph.add_vertex_on_wire(VertexBuilder::z()
            .qubit(control)
            .qubit_coords()
            .build()
        );

        let x = graph.add_vertex_on_wire(VertexBuilder::x()
            .qubit(target)
            .qubit_coords()
            .build()
        );

        graph.add_wires_excluding(0..capacity, [target, control]);
        graph.add_edge(z, x);
        graph
    }

    /// Builder: Generates a BaseGraph instance of a CZ gate
    fn cz(control: usize, target: usize) -> Graph {
        assert_ne!(control, target);
        let capacity = max(control, target) + 1;
        let mut graph = Graph::new(capacity);

        let z1 = graph.add_vertex_on_wire(VertexBuilder::z()
            .qubit(control)
            .qubit_coords()
            .build()
        );

        let z2 = graph.add_vertex_on_wire(VertexBuilder::z()
            .qubit(target)
            .qubit_coords()
            .build()
        );

        graph.add_wires_excluding(0..capacity, [target, control]);
        graph.add_edge(z1, z2);
        graph
    }

    /// Builder: Generates a BaseGraph instance of a z plus gate
    fn z_plus(qubit: usize) -> Graph {
        let capacity = qubit + 1;
        let mut graph = Graph::new(capacity);

        graph.add_wires_excluding(0..capacity, [qubit]);
        graph.add_vertex_on_wire(VertexBuilder::z_plus()
            .qubit(qubit)
            .qubit_coords()
            .build()
        );

        graph
    }

    /// Builder: Generates a BaseGraph instance of a z minus gate
    fn z_minus(qubit: usize) -> Graph {
        let capacity = qubit + 1;
        let mut graph = Graph::new(capacity);

        graph.add_wires_excluding(0..capacity, [qubit]);
        graph.add_vertex_on_wire(VertexBuilder::z_minus()
            .qubit(qubit)
            .qubit_coords()
            .build()
        );

        graph
    }

    /// Builder: Generates a BaseGraph instance of a x plus gate
    fn x_plus(qubit: usize) -> Graph {
        let capacity = qubit + 1;
        let mut graph = Graph::new(capacity);

        graph.add_wires_excluding(0..capacity, [qubit]);
        graph.add_vertex_on_wire(VertexBuilder::x_plus()
            .qubit(qubit)
            .qubit_coords()
            .build()
        );

        graph
    }

    /// Builder: Generates a BaseGraph instance of a x minus gate
    fn x_minus(qubit: usize) -> Graph {
        let capacity = qubit + 1;
        let mut graph = Graph::new(capacity);

        graph.add_wires_excluding(0..capacity, [qubit]);
        graph.add_vertex_on_wire(VertexBuilder::x_minus()
            .qubit(qubit)
            .qubit_coords()
            .build()
        );

        graph
    }

    /// Builder: Generates a BaseGraph instance of a y plus gate
    fn y_plus(qubit: usize) -> Graph {
        let capacity = qubit + 1;
        let mut graph = Graph::new(capacity);

        graph.add_wires_excluding(0..capacity, [qubit]);
        graph.add_vertex_on_wire(VertexBuilder::y_plus()
            .qubit(qubit)
            .qubit_coords()
            .build()
        );

        graph
    }

    /// Builder: Generates a BaseGraph instance of a y minus gate
    fn y_minus(qubit: usize) -> Graph {
        let capacity = qubit + 1;
        let mut graph = Graph::new(capacity);

        graph.add_wires_excluding(0..capacity, [qubit]);
        graph.add_vertex_on_wire(VertexBuilder::y_minus()
            .qubit(qubit)
            .qubit_coords()
            .build()
        );

        graph
    }
}


#[cfg(test)]
mod tests {
    use crate::builders::{Clifford, GraphBuilder};

    #[test]
    fn cx() {
        let cx = GraphBuilder::cx(0, 1);
        assert_eq!(cx.capacity(), 2);
        assert_eq!(cx.num_inputs(), 2);
        assert_eq!(cx.num_outputs(), 2);
        assert_eq!(cx.num_vertices(), 6);
        assert_eq!(cx.num_edges(), 5);
    }

    #[test]
    fn cz() {
        let cz = GraphBuilder::cz(0, 1);
        assert_eq!(cz.capacity(), 2);
        assert_eq!(cz.num_inputs(), 2);
        assert_eq!(cz.num_outputs(), 2);
        assert_eq!(cz.num_vertices(), 6);
        assert_eq!(cz.num_edges(), 5);
    }

    #[test]
    fn z_plus() {
        let z_plus = GraphBuilder::z_plus(0);
        assert_eq!(z_plus.capacity(), 1);
        assert_eq!(z_plus.num_inputs(), 1);
        assert_eq!(z_plus.num_outputs(), 1);
        assert_eq!(z_plus.num_vertices(), 3);
        assert_eq!(z_plus.num_edges(), 2);
    }

    #[test]
    fn x_plus() {
        let x_plus = GraphBuilder::x_plus(0);
        assert_eq!(x_plus.capacity(), 1);
        assert_eq!(x_plus.num_inputs(), 1);
        assert_eq!(x_plus.num_outputs(), 1);
        assert_eq!(x_plus.num_vertices(), 3);
        assert_eq!(x_plus.num_edges(), 2);
    }

    #[test]
    fn y_plus() {
        let y_plus = GraphBuilder::y_plus(0);
        assert_eq!(y_plus.capacity(), 1);
        assert_eq!(y_plus.num_inputs(), 1);
        assert_eq!(y_plus.num_outputs(), 1);
        assert_eq!(y_plus.num_vertices(), 3);
        assert_eq!(y_plus.num_edges(), 2);
    }
}