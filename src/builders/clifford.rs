/*use crate::builders::{Clifford, GraphBuilder};
use crate::graph::{Graph, VertexBuilder};
use std::cmp::max;

impl Clifford for GraphBuilder {
    /// Builder: Generates a BaseGraph instance of a CX gate
    fn cx(control: usize, target: usize) -> Graph {
        assert_ne!(control, target);
        let capacity = max(control, target) + 1;
        let mut graph = Graph::new(capacity);

        let z = graph.add_vertex_along_wire(control, VertexBuilder::z()
            .coords(0.0, control as f64)
            .build()
        );

        let x = graph.add_vertex_along_wire(target, VertexBuilder::x()
            .coords(0.0, target as f64)
            .build()
        );

        graph.add_wires_excluding_to(0..capacity, [target, control]);
        graph.position_inputs();
        graph.position_outputs();
        graph.add_edge(z, x);
        graph
    }

    /// Builder: Generates a BaseGraph instance of a CZ gate
    fn cz(control: usize, target: usize) -> Graph {
        assert_ne!(control, target);
        let capacity = max(control, target) + 1;
        let mut graph = Graph::new(capacity);

        let wire1 = graph.add_wire_to(control);
        let z1 = graph.add_vertex_to_edge(wire1, VertexBuilder::z()
            .coords(0.0, control as f64)
            .build()
        );

        let wire2 = graph.add_wire_to(target);
        let z2 = graph.add_vertex_to_edge(wire2, VertexBuilder::z()
            .coords(0.0, target as f64)
            .build()
        );

        graph.position_inputs();
        graph.position_outputs();
        graph.add_wires_excluding(0..capacity, [target, control]);
        graph.add_edge(z1, z2);
        graph
    }

    /// Builder: Generates a BaseGraph instance of a z plus gate
    fn z_plus(qubit: usize) -> Graph {
        let capacity = qubit + 1;
        let mut graph = Graph::new(capacity);

        graph.add_wires_excluding(0..capacity, [qubit]);
        graph.add_vertex_along_wire(qubit, VertexBuilder::z_plus()
            .coords(0.0, qubit as f64)
            .build()
        );

        graph
    }

    /// Builder: Generates a BaseGraph instance of a z minus gate
    fn z_minus(qubit: usize) -> Graph {
        let capacity = qubit + 1;
        let mut graph = Graph::new(capacity);

        graph.add_wires_excluding(0..capacity, [qubit]);
        graph.add_vertex_along_wire(qubit, VertexBuilder::z_minus()
            .coords(0.0, qubit as f64)
            .build()
        );
        graph
    }

    /// Builder: Generates a BaseGraph instance of a x plus gate
    fn x_plus(qubit: usize) -> Graph {
        let capacity = qubit + 1;
        let mut graph = Graph::new(capacity);

        graph.add_wires_excluding(0..capacity, [qubit]);
        graph.add_vertex_along_wire(qubit, VertexBuilder::x_plus()
            .coords(0.0, qubit as f64)
            .build()
        );

        graph
    }

    /// Builder: Generates a BaseGraph instance of a x minus gate
    fn x_minus(qubit: usize) -> Graph {
        let capacity = qubit + 1;
        let mut graph = Graph::new(capacity);

        graph.add_wires_excluding(0..capacity, [qubit]);
        graph.add_vertex_along_wire(qubit, VertexBuilder::x_minus()
            .coords(0.0, qubit as f64)
            .build()
        );

        graph
    }

    /// Builder: Generates a BaseGraph instance of a y plus gate
    fn y_plus(qubit: usize) -> Graph {
        let capacity = qubit + 1;
        let mut graph = Graph::new(capacity);

        graph.add_wires_excluding(0..capacity, [qubit]);
        graph.add_vertex_along_wire(qubit, VertexBuilder::y_plus()
            .coords(0.0, qubit as f64)
            .build()
        );

        graph
    }

    /// Builder: Generates a BaseGraph instance of a y minus gate
    fn y_minus(qubit: usize) -> Graph {
        let capacity = qubit + 1;
        let mut graph = Graph::new(capacity);

        graph.add_wires_excluding(0..capacity, [qubit]);
        graph.add_vertex_along_wire(qubit, VertexBuilder::y_minus()
            .coords(0.0, qubit as f64)
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
        assert_eq!(cx.num_inputs(), 2);
        assert_eq!(cx.num_outputs(), 2);
        assert_eq!(cx.num_vertices(), 6);
        assert_eq!(cx.num_edges(), 5);
    }

    #[test]
    fn cz() {
        let cz = GraphBuilder::cz(0, 1);
        assert_eq!(cz.num_inputs(), 2);
        assert_eq!(cz.num_outputs(), 2);
        assert_eq!(cz.num_vertices(), 6);
        assert_eq!(cz.num_edges(), 5);
    }

    #[test]
    fn z_plus() {
        let z_plus = GraphBuilder::z_plus(0);
        assert_eq!(z_plus.num_inputs(), 1);
        assert_eq!(z_plus.num_outputs(), 1);
        assert_eq!(z_plus.num_vertices(), 3);
        assert_eq!(z_plus.num_edges(), 2);
    }

    #[test]
    fn x_plus() {
        let x_plus = GraphBuilder::x_plus(0);
        assert_eq!(x_plus.num_inputs(), 1);
        assert_eq!(x_plus.num_outputs(), 1);
        assert_eq!(x_plus.num_vertices(), 3);
        assert_eq!(x_plus.num_edges(), 2);
    }

    #[test]
    fn y_plus() {
        let y_plus = GraphBuilder::y_plus(0);
        assert_eq!(y_plus.num_inputs(), 1);
        assert_eq!(y_plus.num_outputs(), 1);
        assert_eq!(y_plus.num_vertices(), 3);
        assert_eq!(y_plus.num_edges(), 2);
    }
}*/