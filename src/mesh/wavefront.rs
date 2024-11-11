use std::fs::File;
use std::io::prelude::*;

use crate::geometry::Vector3;
use crate::mesh::PolygonSoupMesh;

#[derive(Debug, Clone)]
pub struct ObjReader {
    path: String,
}

impl ObjReader {
    /// Construct an ObjReader from its reference path
    pub fn new(path: &str) -> ObjReader {
        ObjReader {
            path: path.to_string(),
        }
    }

    /// Read the file into a PolygonSoup mesh
    pub fn read(&self) -> std::io::Result<PolygonSoupMesh> {
        let mut file = File::open(&self.path)?;
        let mut mesh = PolygonSoupMesh::new();
        let mut data = String::new();

        // TODO: handle gzip
        file.read_to_string(&mut data)?;

        for line in data.lines() {
            let line = line.trim();
            let args = line.splitn(2, char::is_whitespace).collect::<Vec<&str>>();

            match args.first() {
                Some(&"v") => self.parse_vertex(&mut mesh, &args[1]),
                Some(&"f") => self.parse_face(&mut mesh, &args[1]),
                Some(&"g") => self.parse_group(&mut mesh, &args[1]),
                _ => Ok(()),
            }?;
        }

        Ok(mesh)
    }

    /// Parse a vertex
    fn parse_vertex(&self, mesh: &mut PolygonSoupMesh, data: &str) -> std::io::Result<()> {
        let mut vertex = Vector3::zeros();

        for (i, text) in data.split_whitespace().enumerate() {
            if i >= 3 {
                return Err(ParseObjError::InvalidVertex(data.to_string()).into());
            }

            if let Ok(value) = text.parse::<f64>() {
                vertex[i] = value;
            } else {
                return Err(ParseObjError::InvalidVertex(data.to_string()).into());
            }
        }

        mesh.insert_vertex(vertex);

        Ok(())
    }

    /// Parse a face
    fn parse_face(&self, mesh: &mut PolygonSoupMesh, data: &str) -> std::io::Result<()> {
        let mut vertices = vec![];
        let patch = mesh.n_patches();

        for text in data.split_whitespace() {
            if let Some(text) = text.splitn(2, "/").next() {
                if let Ok(value) = text.parse::<usize>() {
                    if value <= 0 {
                        return Err(ParseObjError::InvalidFace(data.to_string()).into());
                    }

                    vertices.push(value - 1);
                }
            }
        }

        if vertices.len() < 3 {
            return Err(ParseObjError::InvalidFace(data.to_string()).into());
        }

        if patch != 0 {
            mesh.insert_face(&vertices, Some(patch - 1));
        } else {
            mesh.insert_face(&vertices, None);
        }

        Ok(())
    }

    /// Parse a group
    pub fn parse_group(&self, mesh: &mut PolygonSoupMesh, data: &str) -> std::io::Result<()> {
        let name = data.trim();
        mesh.insert_patch(name);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ParseObjError {
    InvalidVertex(String),
    InvalidFace(String),
}

impl std::fmt::Display for ParseObjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseObjError::InvalidVertex(m) => write!(f, "invalid vertex: {}", m),
            ParseObjError::InvalidFace(m) => write!(f, "invalid face: {}", m),
        }
    }
}

impl std::error::Error for ParseObjError {}

impl From<ParseObjError> for std::io::Error {
    fn from(err: ParseObjError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
    }
}
