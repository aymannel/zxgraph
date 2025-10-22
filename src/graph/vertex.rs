use crate::graph::phase::Phase;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VertexType { B, Z, X, Y, H }


#[derive(Debug, Clone, PartialEq)]
pub struct Vertex {
    vertex_type: VertexType,
    phase: Phase,
    qubit: usize,
    x_pos: Option<f64>,
    y_pos: Option<f64>,
}

impl Vertex {
    /// Getter: Returns vertex_type
    pub fn vertex_type(&self) -> VertexType {
        self.vertex_type
    }

    /// Getter: Returns vertex_type
    pub fn phase(&self) -> Phase {
        self.phase
    }

    /// Getter: Returns qubit
    pub fn qubit(&self) -> usize {
        self.qubit
    }

    /// Getter: Returns x coordinate
    pub fn x_pos(&self) -> Option<f64> {
        self.x_pos
    }

    /// Getter: Returns x coordinate
    pub fn y_pos(&self) -> Option<f64> {
        self.y_pos
    }
}


#[derive(Debug, Clone)]
pub struct VertexBuilder {
    vertex_type: Option<VertexType>,
    phase: Option<Phase>,
    qubit: Option<usize>,
    x_pos: Option<f64>,
    y_pos: Option<f64>,
}

impl VertexBuilder {
    pub fn new() -> Self {
        Self {
            vertex_type: None,
            phase: None,
            qubit: None,
            x_pos: None,
            y_pos: None,
        }
    }

    /// Builder: build Vertex
    pub fn build(self) -> Vertex {
        Vertex {
            vertex_type: self.vertex_type.expect("vertex_type not set"),
            phase: self.phase.unwrap_or(Phase::zero()),
            qubit: self.qubit.expect("qubit not set"),
            x_pos: self.x_pos,
            y_pos: self.y_pos,
        }
    }

    /// Builder: set vertex_type
    pub fn vertex_type(mut self, vertex_type: VertexType) -> Self {
        self.vertex_type = Some(vertex_type);
        self
    }

    /// Builder: set phase
    pub fn phase(mut self, phase: Phase) -> Self {
        self.phase = Some(phase);
        self
    }

    /// Builder: set qubit
    pub fn qubit(mut self, qubit: usize) -> Self {
        self.qubit = Some(qubit);
        self
    }

    /// Builder: set x_pos
    pub fn x_pos(mut self, x: f64) -> Self {
        self.x_pos = Some(x);
        self
    }

    /// Builder: set y_pos
    pub fn y_pos(mut self, y: f64) -> Self {
        self.y_pos = Some(y);
        self
    }

    /// Builder: set default coordinates
    pub fn qubit_coords(mut self) -> Self {
        let qubit = self.qubit.expect("qubit must be set before calling default_coords");
        self.y_pos = Some(qubit as f64);
        self.x_pos = Some(0.0);
        self
    }

    /// Builder: convenience constructor
    pub fn z() -> Self {
        Self::new().vertex_type(VertexType::Z)
    }

    /// Builder: convenience constructor
    pub fn x() -> Self {
        Self::new().vertex_type(VertexType::X)
    }

    /// Builder: convenience constructor
    pub fn y() -> Self {
        Self::new().vertex_type(VertexType::Y)
    }

    /// Builder: convenience constructor
    pub fn z_pauli() -> Self {
        Self::z().phase(Phase::one())
    }

    /// Builder: convenience constructor
    pub fn x_pauli() -> Self {
        Self::x().phase(Phase::one())
    }

    /// Builder: convenience constructor
    pub fn y_pauli() -> Self {
        Self::y().phase(Phase::one())
    }

    /// Builder: convenience constructor
    pub fn z_plus() -> Self {
        Self::z().phase(Phase::plus())
    }

    /// Builder: convenience constructor
    pub fn z_minus() -> Self {
        Self::z().phase(Phase::minus())
    }

    /// Builder: convenience constructor
    pub fn x_plus() -> Self {
        Self::x().phase(Phase::plus())
    }

    /// Builder: convenience constructor
    pub fn x_minus() -> Self {
        Self::x().phase(Phase::minus())
    }

    /// Builder: convenience constructor
    pub fn y_plus() -> Self {
        Self::y().phase(Phase::plus())
    }

    /// Builder: convenience constructor
    pub fn y_minus() -> Self {
        Self::y().phase(Phase::minus())
    }

    /// Builder: convenience constructor
    pub fn b() -> Self {
        Self::new().vertex_type(VertexType::B).phase(Phase::zero())
    }
}
