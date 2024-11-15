use std::collections::HashMap;

use crate::geometry::Vector3;
use crate::mesh::{ObjReader, PolygonSoupMesh};

#[derive(Debug, Clone, Default)]
pub struct HeMesh {
    vertices: Vec<HeVertex>,
    faces: Vec<HeFace>,
    half_edges: Vec<HeHalfEdge>,
    patches: Vec<HePatch>,
}

impl HeMesh {
    /// Construct a half edge mesh from a polygon soup mesh
    pub fn new(soup: &PolygonSoupMesh) -> Result<HeMesh, HeMeshError> {
        let mut mesh = HeMesh::default();
        let mut edges = HashMap::<(usize, usize), Vec<usize>>::new();

        // Index the patches
        for patch_id in 0..soup.n_patches() {
            let name = soup.patch(patch_id).to_string();
            let patch = HePatch { name };
            mesh.patches.push(patch);
        }

        // Index the vertices without reference to the half edge
        // originating from the vertex.
        for vertex_id in 0..soup.n_vertices() {
            let position = soup.vertex(vertex_id);
            let vertex = HeVertex {
                position,
                half_edge: 0,
            };
            mesh.vertices.push(vertex);
        }

        // Index the faces and each half edge bounding the face without
        // reference to their twin half edges.
        for face_id in 0..soup.n_faces() {
            let (vertices, patch) = soup.face(face_id);
            let nv = vertices.len();
            let nh = mesh.half_edges.len();

            let face = HeFace {
                half_edge: nh,
                patch: patch,
            };
            mesh.faces.push(face);

            for (k, vertex_id) in vertices.iter().enumerate() {
                let prev = nh + ((k as i32 + nv as i32 - 1) % nv as i32) as usize;
                let next = nh + ((k as i32 + nv as i32 - 1) % nv as i32) as usize;

                let half_edge = HeHalfEdge {
                    origin: *vertex_id,
                    face: face_id,
                    prev: prev,
                    next: next,
                    twin: None,
                };

                mesh.half_edges.push(half_edge);

                if let Some(vertex) = mesh.vertices.get_mut(*vertex_id) {
                    vertex.half_edge = nh + k;
                }

                let ki = *vertex_id;
                let kn = vertices[(k + 1) % nv];
                let ke = (ki.min(kn), ki.max(kn));

                edges
                    .entry(ke)
                    .and_modify(|h| h.push(nh + k))
                    .or_insert(vec![nh + k]);
            }
        }

        for (_, twins) in edges.iter() {
            if twins.len() > 2 {
                return Err(HeMeshError::NonManifold);
            }

            if twins.len() == 2 {
                let mut half_edge = mesh.half_edges[twins[0]];
                half_edge.twin = Some(twins[1]);
                mesh.half_edges[twins[0]] = half_edge;

                let mut half_edge = mesh.half_edges[twins[1]];
                half_edge.twin = Some(twins[0]);
                mesh.half_edges[twins[1]] = half_edge;
            }
        }

        Ok(mesh)
    }

    /// Import a half edge mesh from an OBJ file
    pub fn import_obj(path: &str) -> std::io::Result<HeMesh> {
        let soup = ObjReader::new(&path).read()?;
        let result = HeMesh::new(&soup);

        match result {
            Ok(mesh) => Ok(mesh),
            Err(err) => Err(err.into()),
        }
    }

    /// Export a half edge mesh to an OBJ file
    pub fn export_obj(_path: &str) {
        unimplemented!();
    }

    /// Get a borrowed reference to the vertices
    pub fn vertices(&self) -> &Vec<HeVertex> {
        &self.vertices
    }

    /// Get a borrowed reference to the faces
    pub fn faces(&self) -> &Vec<HeFace> {
        &self.faces
    }

    /// Get a borrowed reference to the half edges
    pub fn half_edges(&self) -> &Vec<HeHalfEdge> {
        &self.half_edges
    }

    /// Get a borrowed reference to the patches
    pub fn patches(&self) -> &Vec<HePatch> {
        &self.patches
    }

    /// Check if the mesh is closed
    pub fn is_closed(&self) -> bool {
        self.half_edges.iter().find(|h| h.is_boundary()).is_none()
    }

    /// Check if all contiguous faces are oriented consistently
    pub fn is_consistent(&self) -> bool {
        self.half_edges
            .iter()
            .filter(|h| !h.is_boundary())
            .find(|h| self.half_edges[h.twin.unwrap()].origin == h.origin)
            .is_none()
    }

    /// Get the contiguous faces as components
    pub fn components(&self) -> Vec<Vec<usize>> {
        unimplemented!();
    }

    /// Orient the mesh
    pub fn orient(&mut self) {
        unimplemented!();
    }

    /// Zip any open edges
    pub fn zip_edges(&mut self) {
        unimplemented!();
    }

    /// Merge naively with another mesh
    pub fn merge(&mut self, _other: &HeMesh) {
        unimplemented!();
    }

    /// Extract the subset of faces into a new mesh
    pub fn extract_face(&self, _faces: &[usize]) -> HeMesh {
        unimplemented!();
    }

    /// Extract the subset of patches by name into a new mesh
    pub fn extract_patches(&self, _patches: &[&str]) -> HeMesh {
        unimplemented!();
    }
}

#[derive(Debug, Copy, Clone)]
pub struct HeVertex {
    position: Vector3,
    half_edge: usize,
}

impl HeVertex {
    /// Get the position
    pub fn position(&self) -> Vector3 {
        self.position
    }

    /// Get the half edge originating at the vertex
    pub fn half_edge(&self) -> usize {
        self.half_edge
    }
}

#[derive(Debug, Copy, Clone)]
pub struct HeFace {
    half_edge: usize,
    patch: Option<usize>,
}

impl HeFace {
    /// Get the starting half edge handle
    pub fn half_edge(&self) -> usize {
        self.half_edge
    }

    /// Get the patch handle
    pub fn patch(&self) -> Option<usize> {
        self.patch
    }
}

#[derive(Debug, Copy, Clone)]
pub struct HeHalfEdge {
    origin: usize,
    face: usize,
    prev: usize,
    next: usize,
    twin: Option<usize>,
}

impl HeHalfEdge {
    /// Get the origin vertex handle
    pub fn origin(&self) -> usize {
        self.origin
    }

    /// Get the incident face handle
    pub fn face(&self) -> usize {
        self.face
    }

    /// Get the previous half edge handle
    pub fn prev(&self) -> usize {
        self.prev
    }

    /// Get the next half edge handle
    pub fn next(&self) -> usize {
        self.next
    }

    /// Get the twin half edge handle (if it exists)
    pub fn twin(&self) -> Option<usize> {
        self.twin
    }

    /// Check if the half edge is on a boundary
    pub fn is_boundary(&self) -> bool {
        self.twin.is_none()
    }
}

#[derive(Debug, Clone)]
pub struct HePatch {
    name: String,
}

impl HePatch {
    /// Get a borrowed reference to the name
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone)]
pub enum HeMeshError {
    NonManifold,
}

impl std::fmt::Display for HeMeshError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HeMeshError::NonManifold => write!(f, "non-manifold mesh"),
        }
    }
}

impl From<HeMeshError> for std::io::Error {
    fn from(err: HeMeshError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
    }
}
