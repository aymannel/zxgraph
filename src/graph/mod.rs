mod graph;
mod vertex;
pub mod phase;

pub use graph::Graph;
pub use vertex::{Vertex, VertexBuilder, VertexType};


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EdgeType { Simple, Hadamard }