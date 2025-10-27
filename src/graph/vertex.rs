use crate::graph::phase::Phase;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VertexType { B, Z, X, Y, H }


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point { pub x: f64, pub y: f64 }


#[derive(Debug, Clone, PartialEq)]
pub struct Vertex {
    vertex_type: VertexType,
    phase: Phase,
    coords: Option<Point>,
}

impl Vertex {
    /// Returns vertex_type
    pub fn vertex_type(&self) -> VertexType {
        self.vertex_type
    }

    /// Returns vertex_type
    pub fn phase(&self) -> Phase {
        self.phase
    }

    /// Returns x coordinate
    pub fn coords(&self) -> Option<Point> {
        self.coords
    }

    /// Set vertex_type
    pub fn set_vertex_type(&mut self, vertex_type: VertexType) {
        self.vertex_type = vertex_type;
    }

    /// Set phase
    pub fn set_phase(&mut self, phase: Phase) {
        self.phase = phase;
    }

    /// Set coordinates
    pub fn set_coords(&mut self, x: f64, y: f64) {
        self.coords = Some(Point {x, y});
    }
}


// todo - use the typestate builder pattern here
#[derive(Debug, Clone)]
pub struct VertexBuilder {
    vertex_type: Option<VertexType>,
    phase: Option<Phase>,
    coords: Option<Point>,
}

impl VertexBuilder {
    pub fn new() -> Self {
        Self {
            vertex_type: None,
            phase: None,
            coords: None,
        }
    }

    /// Builder: build Vertex
    pub fn build(self) -> Vertex {
        Vertex {
            vertex_type: self.vertex_type.expect("vertex_type not set"),
            phase: self.phase.unwrap_or(Phase::zero()),
            coords: self.coords,
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

    /// Builder: set coordinates
    pub fn coords(mut self, x: f64, y: f64) -> Self {
        self.coords = Some(Point {x, y});
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
