use crate::graph::phase::Phase;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VertexType { B, Z, X, Y, H }

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EdgeType { Simple, Hadamard }

#[derive(Debug, Clone)]
pub struct Vertex {
    pub vertex_type: VertexType,
    pub phase: Phase,
    pub qubit: f64,
    pub row: f64,
}