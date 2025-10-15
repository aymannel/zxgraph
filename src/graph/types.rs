use crate::graph::{Bases, Boundary, Clifford, Pauli};
use crate::graph::phase::Phase;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EdgeType { Simple, Hadamard }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VertexType { B, Z, X, Y, H }

#[derive(Debug, Clone)]
pub struct Vertex {
    pub vertex_type: VertexType,
    pub phase: Phase,
    pub qubit: usize,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

impl Vertex {
    pub fn new() -> Self {
        Self {
            vertex_type: VertexType::Z,
            phase: Phase::zero(),
            qubit: 0,
            x: None,
            y: None,
        }
    }

    pub fn with_type(mut self, vertex_type: VertexType) -> Self {
        self.vertex_type = vertex_type;
        self
    }

    pub fn with_phase(mut self, phase: Phase) -> Self {
        self.phase = phase;
        self
    }

    pub fn with_qubit(mut self, qubit: usize) -> Self {
        self.qubit = qubit;
        self
    }

    pub fn with_x(mut self, x: f64) -> Self {
        self.x = Some(x);
        self
    }

    pub fn with_y(mut self, y: f64) -> Self {
        self.y = Some(y);
        self
    }
}

impl Boundary for Vertex {
    fn b() -> Vertex {
        Vertex::new()
            .with_type(VertexType::B)
            .with_phase(Phase::zero())
    }
}

impl Bases for Vertex {
    fn z() -> Vertex {
        Vertex::new()
            .with_type(VertexType::Z)
            .with_phase(Phase::zero())
    }

    fn x() -> Vertex {
        Vertex::new()
            .with_type(VertexType::X)
            .with_phase(Phase::zero())
    }

    fn y() -> Vertex {
        Vertex::new()
            .with_type(VertexType::Y)
            .with_phase(Phase::zero())
    }
}

impl Pauli for Vertex {
    fn z_pauli() -> Vertex {
        Vertex::new()
            .with_type(VertexType::Z)
            .with_phase(Phase::one())
    }

    fn x_pauli() -> Vertex {
        Vertex::new()
            .with_type(VertexType::X)
            .with_phase(Phase::one())
    }

    fn y_pauli() -> Vertex {
        Vertex::new()
            .with_type(VertexType::Y)
            .with_phase(Phase::one())
    }
}

impl Clifford for Vertex {
    fn z_plus() -> Vertex {
        Vertex::new()
            .with_type(VertexType::Z)
            .with_phase(Phase::plus())
    }

    fn z_minus() -> Vertex {
        Vertex::new()
            .with_type(VertexType::Z)
            .with_phase(Phase::minus())
    }

    fn x_plus() -> Vertex {
        Vertex::new()
            .with_type(VertexType::X)
            .with_phase(Phase::plus())
    }

    fn x_minus() -> Vertex {
        Vertex::new()
            .with_type(VertexType::X)
            .with_phase(Phase::minus())
    }

    fn y_plus() -> Vertex {
        Vertex::new()
            .with_type(VertexType::Y)
            .with_phase(Phase::plus())
    }

    fn y_minus() -> Vertex {
        Vertex::new()
            .with_type(VertexType::Y)
            .with_phase(Phase::minus())
    }
}