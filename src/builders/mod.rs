mod pauli;
mod clifford;
mod gadget;

use crate::graph::phase::Phase;
use crate::graph::Graph;


pub struct GraphBuilder;

pub trait Gadget {
    /// Creates an instance of BaseGraph representing a Pauli Gadget
    fn gadget(pauli_string: &str, phase: Phase) -> Graph;
}

pub trait Pauli {
    /// Creates an instance of BaseGraph representing Pauli Z
    fn z_pauli(qubit: usize) -> Graph;

    /// Creates an instance of BaseGraph representing Pauli X
    fn x_pauli(qubit: usize) -> Graph;

    /// Creates an instance of BaseGraph representing Pauli Y
    fn y_pauli(qubit: usize) -> Graph;
}

pub trait Clifford {
    /// Creates an instance of BaseGraph representing the CX Clifford
    fn cx(control: usize, target: usize) -> Graph;

    /// Creates an instance of BaseGraph representing the CZ Clifford
    fn cz(control: usize, target: usize) -> Graph;

    /// Creates an instance of BaseGraph representing the Z Minus Clifford
    fn z_plus(qubit: usize) -> Graph;

    /// Creates an instance of BaseGraph representing the Z Minus Clifford
    fn z_minus(qubit: usize) -> Graph;

    /// Creates an instance of BaseGraph representing the X Plus Clifford
    fn x_plus(qubit: usize) -> Graph;

    /// Creates an instance of BaseGraph representing the X Minus Clifford
    fn x_minus(qubit: usize) -> Graph;

    /// Creates an instance of BaseGraph representing the Y Minus Clifford
    fn y_plus(qubit: usize) -> Graph;

    /// Creates an instance of BaseGraph representing the Y Minus Clifford
    fn y_minus(qubit: usize) -> Graph;
}