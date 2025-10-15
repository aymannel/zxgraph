mod graph;
mod types;
pub mod phase;

mod pauli;
mod gadget;
mod clifford;

use crate::graph::phase::Phase;
pub use graph::BaseGraph;
pub use types::{EdgeType, Vertex, VertexType};

pub trait Boundary {
    fn b() -> Vertex;
}

pub trait Bases {
    fn z() -> Vertex;
    fn x() -> Vertex;
    fn y() -> Vertex;
}

pub trait Pauli {
    fn z_pauli() -> Vertex;
    fn x_pauli() -> Vertex;
    fn y_pauli() -> Vertex;
}

pub trait Clifford {
    fn z_plus() -> Vertex;
    fn z_minus() -> Vertex;
    fn x_plus() -> Vertex;
    fn x_minus() -> Vertex;
    fn y_plus() -> Vertex;
    fn y_minus() -> Vertex;
}

pub trait GadgetGraph {
    /// Creates an instance of BaseGraph representing a Pauli Gadget
    fn gadget(pauli_string: &str, phase: Phase) -> BaseGraph;
}

pub trait PauliGraph {
    /// Creates an instance of BaseGraph representing Pauli Z
    fn z_pauli(qubit: usize) -> BaseGraph;

    /// Creates an instance of BaseGraph representing Pauli X
    fn x_pauli(qubit: usize) -> BaseGraph;

    /// Creates an instance of BaseGraph representing Pauli Y
    fn y_pauli(qubit: usize) -> BaseGraph;
}

pub trait CliffordGraph {
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
