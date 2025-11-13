use thiserror::Error;

pub mod export;

#[derive(Debug, Error)]
pub enum ExportError {
    #[error("I/O error: {0}")] Io(#[from] std::io::Error),
    #[error("formatting error: {0}")] Fmt(#[from] std::fmt::Error),
    #[error("vertex {0} has no coordinates")] MissingCoords(usize),
}

pub trait Exportable {
    /// Generates latex
    fn to_tex(&self, name: &str) -> Result<String, ExportError>;
}
