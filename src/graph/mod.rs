mod graph;
mod types;
pub mod phase;

mod pauli;
mod gadget;
mod clifford;

pub use graph::BaseGraph;
pub use types::{EdgeType, Vertex, VertexType};

pub trait Gadget {
    /// Creates an instance of BaseGraph representing a Pauli Gadget
    fn gadget(pauli_string: &str, phase: f64) -> BaseGraph;
}

pub trait Pauli {
    /// Creates an instance of BaseGraph representing Pauli Z
    fn z(qubit: usize) -> BaseGraph;

    /// Creates an instance of BaseGraph representing Pauli X
    fn x(qubit: usize) -> BaseGraph;

    /// Creates an instance of BaseGraph representing Pauli Y
    fn y(qubit: usize) -> BaseGraph;
}

pub trait Clifford {
    /// Creates an instance of BaseGraph representing the CNOT Clifford
    fn cx(control: usize, target: usize) -> BaseGraph;

    /// Creates an instance of BaseGraph representing the CZ Clifford
    fn cz(control: usize, target: usize) -> BaseGraph;

    /// Creates an instance of BaseGraph representing the Z Minus Clifford
    fn z_plus(qubit: usize) -> BaseGraph;

    /// Creates an instance of BaseGraph representing the Z Minus Clifford
    fn z_minus(qubit: usize) -> BaseGraph;

    /// Creates an instance of BaseGraph representing the X Plus Clifford
    fn x_plus(qubit: usize) -> BaseGraph;

    /// Creates an instance of BaseGraph representing the X Minus Clifford
    fn x_minus(qubit: usize) -> BaseGraph;

    /// Creates an instance of BaseGraph representing the Y Minus Clifford
    fn y_plus(qubit: usize) -> BaseGraph;

    /// Creates an instance of BaseGraph representing the Y Minus Clifford
    fn y_minus(qubit: usize) -> BaseGraph;
}
