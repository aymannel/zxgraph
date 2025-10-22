pub mod export;

pub trait Exportable {
    /// Generates latex
    fn to_tex(&self, name: &str) -> Result<String, Box<dyn std::error::Error>>;
}