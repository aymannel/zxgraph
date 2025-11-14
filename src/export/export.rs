use crate::export::{ExportError, Exportable};
use crate::graph::phase::Phase;
use crate::graph::{Coords, EdgeType, Graph, VertexType};
use petgraph::prelude::EdgeRef;
use std::fmt::Write;
use std::fs;

macro_rules! write_vertex {
    ($target:expr, $style:expr, $index:expr, $coords:expr, $phase:expr) => {
        writeln!(
            $target,
            "\t\t\t\\node [style={}] ({}) at ({}) {{{}}};",
            $style,
            $index,
            $coords,
            $phase,
        )?
    };
}

macro_rules! write_edge {
    ($target:expr, $style:expr, $source:expr, $destination:expr) => {
        writeln!(
            $target,
            "\t\t\t\\draw [style={}] ({}) to ({});",
            $style,
            $source,
            $destination,
        )?
    };
}

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

impl Exportable for VertexType {
    fn to_tex(&self) -> Result<String, ExportError> {
        match &self {
            VertexType::Z => Ok("z_node".to_owned()),
            VertexType::X => Ok("x_node".to_owned()),
            VertexType::Y => Ok("y_node".to_owned()),
            VertexType::H => Ok("hadamard".to_owned()),
        }
    }
}

impl Exportable for Graph {
    fn to_tex(&self) -> Result<String, ExportError> {
        // Write vertices
        let mut vertices = String::new();
        for (index, vertex) in self.enumerate_vertices() {
            if let Some(coords) = vertex.coords() {
                let phase = vertex.phase().to_tex()?;
                let vertex_type = vertex.vertex_type().to_tex()?;
                let style = vertex_type + if phase.is_empty() {""} else {"_phase"};
                write_vertex!(&mut vertices, style, index.index(), coords.to_tex()?, phase);
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
            write_edge!(&mut edges, style, source, target);
        }

        // Write boundaries
        for qubit in 0..self.max_qubit() {
            let input_boundary;
            let output_boundary;
            let y = -(qubit as f64);
            match (self.input_index(qubit), self.output_index(qubit)) {
                (Some(input), Some(output)) => {
                    input_boundary = format!("in{}", qubit);
                    output_boundary = format!("out{}", qubit);
                    write_edge!(&mut edges, "simple_edge", input.index(), input_boundary);
                    write_edge!(&mut edges, "simple_edge", output.index(), output_boundary);
                }
                (None, None) => {
                    input_boundary = format!("wire_in{}", qubit);
                    output_boundary = format!("wire_out{}", qubit);
                    write_edge!(&mut edges, "simple_edge", input_boundary, output_boundary);
                }
                _ => return Err(ExportError::QubitInputOutputMismatch(qubit))
            }
            write_vertex!(&mut vertices, "boundary", input_boundary, format!("-1, {y}"), "");
            write_vertex!(&mut vertices, "boundary", output_boundary, format!("1, {y}"), "");
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
        let graph = GraphBuilder::gadget("YXIZ", Phase::minus());
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