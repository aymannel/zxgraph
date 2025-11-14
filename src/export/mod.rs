use thiserror::Error;

pub mod export;

#[derive(Debug, Error)]
pub enum ExportError {
    #[error("invalid phase fraction")] InvalidPhase,
    #[error("vertex {0} has no coordinates")] VertexMissingCoords(usize),
    #[error("qubit {0} has mismatching input/output")] QubitInputOutputMismatch(usize),
    #[error("formatting error: {0}")] Fmt(#[from] std::fmt::Error),
    #[error("I/O error: {0}")] Io(#[from] std::io::Error),
}

pub trait Exportable {
    /// Generates latex
    fn to_tex(&self) -> Result<String, ExportError>;
}
