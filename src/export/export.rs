use crate::export::Exportable;
use crate::graph::{BaseGraph, EdgeType, VertexType};
use petgraph::prelude::EdgeRef;
use std::error::Error;
use std::fmt::Write;
use std::fs;
use std::process::Command;

impl Exportable for BaseGraph {
    fn to_tex(&self, name: &str) -> Result<(), Box<dyn Error>> {
        // Add vertices
        let mut vertices = String::new();
        for (node_index, vertex) in self.enumerate_vertices() {
            let style: &str = match (vertex.vertex_type, vertex.phase.is_zero()) {
                (VertexType::B, _) => "boundary",
                (VertexType::H, _) => "hadamard",
                (VertexType::Z, true) => "z_node",
                (VertexType::X, true) => "x_node",
                (VertexType::Y, true) => "y_node",
                (VertexType::Z, false) => "z_phase",
                (VertexType::X, false) => "x_phase",
                (VertexType::Y, false) => "y_phase",
            };

            // Format export node
            let row = vertex.row;
            let qubit = -vertex.qubit;
            let index = node_index.index();
            let phase = vertex.phase.to_latex();
            writeln!(&mut vertices, "\t\t\t\\node [style={style}] ({index}) at ({row:.2}, {qubit:.2}) {{{phase}}};")?;
        }

        // Add edges
        let mut edges = String::new();
        for edge in self.enumerate_edges() {
            let style = match edge.weight() {
                EdgeType::Simple => "simple_edge",
                EdgeType::Hadamard => "hadamard_edge",
            };

            // Format export edge
            let source = edge.source().index();
            let target = edge.target().index();
            writeln!(&mut edges, "\t\t\t\\draw [style={style}] ({source}) to ({target});")?;
        }

        // Generate tex
        assert!(name.ends_with(".tex"));
        let output_path = format!("output/{name}");
        let template = fs::read_to_string("src/export/template.tex")?;
        fs::write(&output_path, template
            .replace("{vertices}", &vertices)
            .replace("{edges}", &edges))?;
        Ok(())
    }

    fn to_pdf(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let source_path = format!("output/{name}");
        Command::new("pdflatex").args(&[
            "-interaction=nonstopmode",
            "-halt-on-error",
            "-output-directory", "output",
            &source_path
        ]).status()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{BaseGraph, Clifford, Gadget, Pauli};

    #[test]
    fn can_export_gadget() {
        let file_name = "gadget.tex";
        let graph = BaseGraph::gadget("IXZYY", 1.5);
        graph.to_tex(file_name).expect("Could not generate tex file");
        graph.to_pdf(file_name).expect("Could not generate pdf");

        Command::new("open")
            .current_dir("output").arg(file_name.replace("tex", "pdf"))
            .status().expect("Could not open pdf");
    }

    #[test]
    fn can_export_pauli_x() {
        let file_name = "pauli_x.tex";
        let graph = BaseGraph::y(1);
        graph.to_tex(file_name).expect("Could not generate tex file");
        graph.to_pdf(file_name).expect("Could not generate pdf");

        Command::new("open")
            .current_dir("output").arg(file_name.replace("tex", "pdf"))
            .status().expect("Could not open pdf");
    }

    #[test]
    fn can_export_x_plus() {
        let file_name = "x_plus.tex";
        let graph = BaseGraph::x_plus(1);
        graph.to_tex(file_name).expect("Could not generate tex file");
        graph.to_pdf(file_name).expect("Could not generate pdf");

        Command::new("open")
            .current_dir("output").arg(file_name.replace("tex", "pdf"))
            .status().expect("Could not open pdf");
    }
}