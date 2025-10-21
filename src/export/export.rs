use crate::export::Exportable;
use crate::graph::{Graph, EdgeType, VertexType};
use petgraph::prelude::EdgeRef;
use std::error::Error;
use std::fmt::Write;
use std::fs;
use std::process::Command;

impl Exportable for Graph {
    fn to_tex(&self, name: &str) -> Result<(), Box<dyn Error>> {
        // Add vertices
        let mut vertices = String::new();
        for (node_index, vertex) in self.enumerate_vertices() {
            let style: &str = match (vertex.vertex_type(), vertex.phase().is_zero()) {
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
            let x = vertex.x_pos().unwrap();
            let y = -vertex.y_pos().unwrap();
            let index = node_index.index();
            let phase = vertex.phase().to_latex();
            writeln!(&mut vertices, "\t\t\t\\node [style={style}] ({index}) at ({x:.2}, {y:.2}) {{{phase}}};")?;
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

    // todo - make OS agnostic + check pdflatex is installed // or remove entirely?
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

    fn to_json(&self, name: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builders::{Clifford, Gadget, GraphBuilder, Pauli};
    use crate::graph::phase::Phase;

    #[macro_export]
    macro_rules! export_and_open {
        ($graph:expr, $file_name:expr) => {
            {
                use std::process::Command;
                let file_name = $file_name;
                let graph = $graph;

                graph.to_tex(file_name).expect("Could not generate tex file");
                graph.to_pdf(file_name).expect("Could not generate pdf");

                Command::new("open")
                    .current_dir("output")
                    .arg(file_name.replace("tex", "pdf"))
                    .status()
                    .expect("Could not open pdf");
            }
        };
    }

    #[test]
    fn can_export_gadget() {
        let graph = GraphBuilder::gadget("XYZ", Phase::minus());
        export_and_open!(graph, "gadget.tex");
    }

    #[test]
    fn can_export_pauli_y() {
        let graph = GraphBuilder::pauli_y(1);
        export_and_open!(graph, "pauli_y.tex");
    }

    #[test]
    fn can_export_x_plus() {
        let graph = GraphBuilder::x_plus(1);
        export_and_open!(graph, "x_plus.tex");
    }

    #[test]
    fn can_export_cx() {
        let graph = GraphBuilder::cx(0, 2);
        export_and_open!(graph, "cx.tex");
    }

    #[test]
    fn can_export_cz() {
        let graph = GraphBuilder::cz(1, 2);
        export_and_open!(graph, "cz.tex");
    }
}