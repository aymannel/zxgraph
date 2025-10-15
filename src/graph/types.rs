use crate::graph::phase::Phase;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EdgeType { Simple, Hadamard }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VertexType { B, Z, X, Y, H }

#[derive(Debug, Clone)]
pub struct Vertex {
    pub vertex_type: VertexType,
    pub phase: Phase,
    pub qubit: f64,
    pub row: f64,
}

impl Vertex {
    pub fn new() -> Self {
        Self {
            vertex_type: VertexType::Z,
            phase: Phase::new(),
            qubit: 0.0,
            row: 0.0,
        }
    }

    pub fn z() -> Vertex {
        Vertex::new()
            .with_type(VertexType::Z)
            .with_qubit(1.0)
    }

    pub fn x() -> Vertex {
        Vertex::new()
            .with_type(VertexType::X)
            .with_qubit(1.0)
    }

    pub fn y() -> Vertex {
        Vertex::new()
            .with_type(VertexType::Y)
            .with_qubit(1.0)
    }
    
    pub fn with_type(mut self, vertex_type: VertexType) -> Self {
        self.vertex_type = vertex_type;
        self
    }

    pub fn with_phase(mut self, phase: Phase) -> Self {
        self.phase = phase;
        self
    }

    pub fn with_qubit(mut self, qubit: f64) -> Self {
        self.qubit = qubit;
        self
    }

    pub fn with_row(mut self, row: f64) -> Self {
        self.row = row;
        self
    }
}
