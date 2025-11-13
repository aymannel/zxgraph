use crate::export::{ExportError, Exportable};
use crate::graph::phase::Phase;
use crate::graph::{Coords, EdgeType, Graph, VertexType};
use petgraph::prelude::EdgeRef;
use std::fmt::Write;
use std::fs;

impl Exportable for Coords {
    fn to_tex(&self) -> Result<String, ExportError> {
        Ok(format!("{:.2}, {:.2}", self.x, -self.y))
    }
}

impl Exportable for Phase {
    fn to_tex(&self) -> Result<String, ExportError> {
        match (self.angle().numer(), self.angle().denom()) {
            (Some(0), Some(1)) => Ok(String::new()),
            (Some(1), Some(1)) => Ok("$\\pi$".to_owned()),
            (Some(0), Some(d)) => Ok(format!("$\\frac{{\\pi}}{{{}}}$", d)),
            (Some(n), Some(d)) => Ok(format!("$\\frac{{{}\\pi}}{{{}}}$", n, d)),
            _ => Err(ExportError::InvalidPhase)
        }
    }
}

impl Exportable for Graph {
    fn to_tex(&self) -> Result<String, ExportError> {
        // Write vertices
        let mut vertices = String::new();
        for (index, vertex) in self.enumerate_vertices() {
            if let Some(coords) = vertex.coords() {
                let style = match (vertex.vertex_type(), vertex.phase().is_zero()) {
                    (VertexType::H, _) => "hadamard",
                    (VertexType::Z, true) => "z_node",
                    (VertexType::X, true) => "x_node",
                    (VertexType::Y, true) => "y_node",
                    (VertexType::Z, false) => "z_phase",
                    (VertexType::X, false) => "x_phase",
                    (VertexType::Y, false) => "y_phase",
                };
                writeln!(
                    &mut vertices, "\t\t\t\\node [style={style}] ({}) at ({}) {{{}}};",
                    index.index(),
                    coords.to_tex()?,
                    vertex.phase().to_tex()?
                )?;
            } else {
                return Err(ExportError::VertexMissingCoords(index.index()))
            }
        }

        // Write edges
        let mut edges = String::new();
        for edge in self.enumerate_edges() {
            let source = edge.source().index();
            let target = edge.target().index();
            let style = match edge.weight() {
                EdgeType::Simple => "simple_edge",
                EdgeType::Hadamard => "hadamard_edge",
            };
            writeln!(&mut edges, "\t\t\t\\draw [style={style}] ({source}) to ({target});")?;
        }

        // Write boundaries
        for qubit in 0..self.max_qubit() {
            let y = -(qubit as f64);
            match (self.input_index(qubit), self.output_index(qubit)) {
                (Some(input), Some(output)) => {
                    let input_index = input.index();
                    let output_index = output.index();
                    writeln!(&mut vertices, "\t\t\t\\node [style=boundary] (in{input_index}) at (-1, {y}) {{}};")?;
                    writeln!(&mut vertices, "\t\t\t\\node [style=boundary] (out{output_index}) at (1, {y}) {{}};")?;
                    writeln!(&mut edges, "\t\t\t\\draw [style=simple_edge] ({input_index}) to (in{input_index});")?;
                    writeln!(&mut edges, "\t\t\t\\draw [style=simple_edge] ({output_index}) to (out{output_index});")?;
                },
                (None, None) => {
                    writeln!(&mut vertices, "\t\t\t\\node [style=boundary] (bi{qubit}) at (-1, {y}) {{}};")?;
                    writeln!(&mut vertices, "\t\t\t\\node [style=boundary] (bo{qubit}) at (1, {y}) {{}};")?;
                    writeln!(&mut edges, "\t\t\t\\draw [style=simple_edge] (bi{qubit}) to (bo{qubit});")?;
                }
                _ => todo!()
            }
        }

        // Format latex string
        let template = fs::read_to_string("src/export/template.tex")?;
        let tex_output = template.replace("{vertices}", &vertices).replace("{edges}", &edges);
        Ok(tex_output)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::builders::{Gadget, GraphBuilder};
    use crate::graph::phase::Phase;
    use std::process::Command;

    #[macro_export]
    macro_rules! export_and_open {
        ($graph:expr, $file_name:expr) => {
            {
                let graph = $graph;
                let file_name = $file_name;
                let output = graph.to_tex().expect("Could not generate tex file");

                assert!(file_name.ends_with(".tex"));
                let path = format!("output/{file_name}");

                std::fs::write(&path, output).expect("Failed to write tex file");

                std::process::Command::new("pdflatex").args(&[
                    "-interaction=nonstopmode",
                    "-halt-on-error",
                    "-output-directory", "output",
                    &path
                ]).status().expect("Failed to execute pdflatex");

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
        let graph = GraphBuilder::gadget("YIXZ", Phase::minus());
        export_and_open!(graph, "gadget.tex");
    }

    // #[test]
    // fn can_export_pauli_y() {
    //     let graph = GraphBuilder::pauli_y(1);
    //     export_and_open!(graph, "pauli_y.tex");
    // }

    // #[test]
    // fn can_export_x_plus() {
    //     let graph = GraphBuilder::x_plus(1);
    //     export_and_open!(graph, "x_plus.tex");
    // }

    // #[test]
    // fn can_export_cx() {
    //     let graph = GraphBuilder::cx(0, 2);
    //     export_and_open!(graph, "cx.tex");
    // }

    // #[test]
    // fn can_export_cz() {
    //     let graph = GraphBuilder::cz(1, 2);
    //     export_and_open!(graph, "cz.tex");
    // }

    #[test]
    fn panics_if_missing_coords() {}
}