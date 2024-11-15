use std::collections::HashMap;

use crate::geometry::Vector3;
use crate::mesh::ObjReader;

#[derive(Debug, Clone, Default)]
pub struct HeMesh {
    vertices: Vec<HeVertex>,
    faces: Vec<HeFace>,
    half_edges: Vec<HeHalfEdge>,
    patches: Vec<HePatch>,
}

impl HeMesh {
    /// Import a half edge mesh from an OBJ file
    pub fn import_obj(path: &str) -> std::io::Result<HeMesh> {
        let soup = ObjReader::new(path).read()?;
        let mut mesh = HeMesh::default();
        let mut edges = HashMap::<(usize, usize), Vec<usize>>();

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

        Ok(mesh)
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
