use crate::graph::{Bases, Boundary, Clifford, Pauli};
use crate::graph::phase::Phase;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EdgeType { Simple, Hadamard }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VertexType { B, Z, X, Y, H }

#[derive(Debug, Clone)]
pub struct Vertex {
    vertex_type: VertexType,
    phase: Phase,
    qubit: usize,
    xpos: Option<f64>,
    ypos: Option<f64>,
}

impl Vertex {
    /// Constructor: Creates instance of Vertex with default values
    pub fn new() -> Self {
        Self {
            vertex_type: VertexType::Z,
            phase: Phase::zero(),
            qubit: 0,
            xpos: None,
            ypos: None,
        }
    }

    /// Getter: Returns vertex_type
    pub fn vertex_type(&self) -> &VertexType {
        &self.vertex_type
    }

    /// Getter: Returns vertex_type
    pub fn phase(&self) -> &Phase {
        &self.phase
    }

    /// Getter: Returns qubit
    pub fn qubit(&self) -> usize {
        self.qubit
    }

    /// Getter: Returns x coordinate
    pub fn xpos(&self) -> Option<f64> {
        self.xpos
    }

    /// Getter: Returns x coordinate
    pub fn ypos(&self) -> Option<f64> {
        self.ypos
    }

    /// Builder: Sets vertex_type
    pub fn with_type(mut self, vertex_type: VertexType) -> Self {
        self.vertex_type = vertex_type;
        self
    }

    /// Builder: Sets phase
    pub fn with_phase(mut self, phase: Phase) -> Self {
        self.phase = phase;
        self
    }

    /// Builder: Sets qubit
    pub fn with_qubit(mut self, qubit: usize) -> Self {
        self.qubit = qubit;
        self
    }

    /// Builder: Sets x coordinate
    pub fn with_x(mut self, x: f64) -> Self {
        self.xpos = Some(x);
        self
    }

    /// Builder: Sets y coordinate
    pub fn with_y(mut self, y: f64) -> Self {
        self.ypos = Some(y);
        self
    }

    /// Builder: Sets both coordinates
    pub fn with_coords(self, x: f64, y: f64) -> Self {
        self.with_x(x).with_y(y)
    }
    
    /// Builder: Sets coordinates by qubit
    pub fn with_quords(self) -> Self {
        let qubit = self.qubit();
        self.with_x(1.0).with_y(qubit as f64)
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