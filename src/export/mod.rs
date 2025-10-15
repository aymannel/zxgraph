pub mod export;

pub trait Exportable {
    /// Export to latex
    fn to_tex(&self, name: &str) -> Result<(), Box<dyn std::error::Error>>;

    /// Export to pdf
    fn to_pdf(&self, name: &str) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Export to JSON
    fn to_json(&self, name: &str) -> Result<(), Box<dyn std::error::Error>>;
}