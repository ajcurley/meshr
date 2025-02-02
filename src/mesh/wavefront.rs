use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

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

        if is_gzip(&self.path) {
            let mut file = GzDecoder::new(file);
            file.read_to_string(&mut data)?;
        } else {
            file.read_to_string(&mut data)?;
        }

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
pub struct ObjWriter {
    vertices: Vec<Vector3>,
    faces: Vec<Vec<usize>>,
    face_groups: Vec<Option<usize>>,
    lines: Vec<Vec<usize>>,
    groups: Vec<String>,
}

impl ObjWriter {
    /// Construct a default ObjWriter
    pub fn new() -> ObjWriter {
        ObjWriter {
            vertices: vec![],
            faces: vec![],
            face_groups: vec![],
            lines: vec![],
            groups: vec![],
        }
    }

    /// Set the vertices
    pub fn set_vertices(&mut self, vertices: Vec<Vector3>) {
        self.vertices = vertices;
    }

    /// Set the faces
    pub fn set_faces(&mut self, faces: Vec<Vec<usize>>) {
        self.faces = faces;
    }

    /// Set the face groups
    pub fn set_face_groups(&mut self, face_groups: Vec<Option<usize>>) {
        self.face_groups = face_groups;
    }

    /// Set the lines
    pub fn set_lines(&mut self, lines: Vec<Vec<usize>>) {
        self.lines = lines;
    }

    /// Set the groups
    pub fn set_groups(&mut self, groups: Vec<String>) {
        self.groups = groups;
    }

    /// Write the data to file
    pub fn write(&self, path: &str) -> std::io::Result<()> {
        let mut content = String::new();
        content.push_str(&self.format_vertices());
        content.push_str(&self.format_lines());
        content.push_str(&self.format_faces());

        let mut file = File::create(path)?;
        let data = content.as_bytes();

        if is_gzip(path) {
            let mut encoder = GzEncoder::new(&mut file, Compression::default());
            encoder.write_all(&data)?;
        } else {
            file.write_all(&data)?;
        }

        Ok(())
    }

    /// Format the vertices into a string
    fn format_vertices(&self) -> String {
        self.vertices
            .iter()
            .map(|v| format!("v {} {} {}\n", v[0], v[1], v[2]))
            .collect::<Vec<String>>()
            .join("")
    }

    /// Format the lines into a string
    fn format_lines(&self) -> String {
        let mut content = String::new();

        for l in self.lines.iter() {
            let vertices = l
                .iter()
                .map(|v| (v + 1).to_string())
                .collect::<Vec<String>>()
                .join(" ");

            let entry = format!("l {}\n", vertices);
            content.push_str(&entry);
        }

        content
    }

    /// Format the faces into a string
    fn format_faces(&self) -> String {
        let mut content = String::new();
        let mut index = HashMap::<Option<usize>, Vec<usize>>::new();
        let n = self.faces.len();

        if self.face_groups.is_empty() {
            index.insert(None, (0..n).collect());
        } else {
            for (face_id, &group) in self.face_groups.iter().enumerate() {
                index
                    .entry(group)
                    .and_modify(|f| f.push(face_id))
                    .or_insert(vec![face_id]);
            }
        }

        let mut groups = self
            .groups
            .iter()
            .map(|n| Some(n.to_string()))
            .collect::<Vec<Option<String>>>();

        groups.insert(0, None);

        for (i, group) in groups.iter().enumerate() {
            let mut group_id = None;

            if let Some(name) = group {
                let entry = format!("g {}\n", name);
                content.push_str(&entry);
                group_id = Some(i - 1);
            }

            if let Some(face_ids) = index.remove(&group_id) {
                for face_id in face_ids.iter() {
                    let vertices = self.faces[*face_id]
                        .iter()
                        .map(|v| (v + 1).to_string())
                        .collect::<Vec<String>>()
                        .join(" ");

                    let entry = format!("f {}\n", vertices);
                    content.push_str(&entry);
                }
            }
        }

        content
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

/// Check if a filepathis GZIP
fn is_gzip(path: &str) -> bool {
    let path = Path::new(path);

    if let Some(ext) = path.extension().and_then(OsStr::to_str) {
        let ext = ext.to_lowercase();
        return ext == "gz" || ext == "gzip";
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read() {
        let path = "tests/fixtures/box.obj";
        let mesh = ObjReader::new(&path).read().unwrap();

        assert_eq!(8, mesh.n_vertices());
        assert_eq!(12, mesh.n_faces());
        assert_eq!(0, mesh.n_patches());
    }

    #[test]
    fn read_gzip() {
        let path = "tests/fixtures/box.obj.gz";
        let mesh = ObjReader::new(&path).read().unwrap();

        assert_eq!(8, mesh.n_vertices());
        assert_eq!(12, mesh.n_faces());
        assert_eq!(0, mesh.n_patches());
    }

    #[test]
    fn read_groups() {
        let path = "tests/fixtures/box.groups.obj";
        let mesh = ObjReader::new(&path).read().unwrap();

        assert_eq!(8, mesh.n_vertices());
        assert_eq!(7, mesh.n_faces());
        assert_eq!(6, mesh.n_patches());
    }
}
