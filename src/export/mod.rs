pub mod export;

pub trait Exportable {
    /// Generates tex file
    fn to_tex(&self, name: &str) -> Result<(), Box<dyn std::error::Error>>;

    /// Generates pdf file
    fn to_pdf(&self, name: &str) -> Result<(), Box<dyn std::error::Error>>;
}