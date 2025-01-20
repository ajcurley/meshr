use std::collections::{HashMap, HashSet};

use crate::geometry::{Aabb, Vector3};
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
            let origin = soup.vertex(vertex_id);
            let vertex = HeVertex {
                origin,
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

            mesh.faces.push(HeFace {
                half_edge: nh,
                patch: patch,
            });

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

                // Index the shared half edges. If two half edges are already indexed
                // for the same vertex pair, the mesh must be non-manifold.
                if let Some(shared) = edges.get_mut(&ke) {
                    if shared.len() >= 2 {
                        return Err(HeMeshError::NonManifold);
                    }
                    shared.push(nh + k);
                } else {
                    edges.insert(ke, vec![nh + k]);
                }
            }
        }

        // Index the half edge twins using the shared edges
        for (_, shared) in edges.iter() {
            if shared.len() == 2 {
                mesh.half_edges[shared[0]].twin = Some(shared[1]);
                mesh.half_edges[shared[1]].twin = Some(shared[0]);
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

    /// Get the number of vertices
    pub fn n_vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Get a borrowed reference to the vertices
    pub fn vertices(&self) -> &Vec<HeVertex> {
        &self.vertices
    }

    /// Get a vertex by index
    pub fn vertex(&self, index: usize) -> HeVertex {
        self.vertices[index]
    }

    /// Get the number of faces
    pub fn n_faces(&self) -> usize {
        self.faces.len()
    }

    /// Get a borrowed reference to the faces
    pub fn faces(&self) -> &Vec<HeFace> {
        &self.faces
    }

    /// Get a face by index
    pub fn face(&self, index: usize) -> HeFace {
        self.faces[index]
    }

    /// Get the number of half edges
    pub fn n_half_edges(&self) -> usize {
        self.half_edges.len()
    }

    /// Get a borrowed reference to the half edges
    pub fn half_edges(&self) -> &Vec<HeHalfEdge> {
        &self.half_edges
    }

    /// Get a the half edge by index
    pub fn half_edge(&self, index: usize) -> HeHalfEdge {
        self.half_edges[index]
    }

    /// Get the number of patches
    pub fn n_patches(&self) -> usize {
        self.patches.len()
    }

    /// Get a borrowed reference to the patches
    pub fn patches(&self) -> &Vec<HePatch> {
        &self.patches
    }

    /// Get a patch by index
    pub fn patch(&self, index: usize) -> HePatch {
        self.patches[index].clone()
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

    /// Get the axis-aligned bounding box
    pub fn bounds(&self) -> Aabb {
        let mut min = Vector3::ones() * f64::INFINITY;
        let mut max = Vector3::ones() * f64::NEG_INFINITY;

        for vertex in self.vertices.iter() {
            for i in 0..3 {
                if vertex.origin[i] < min[i] {
                    min[i] = vertex.origin[i];
                } else if vertex.origin[i] > max[i] {
                    max[i] = vertex.origin[i]
                }
            }
        }

        Aabb::from_bounds(min, max)
    }

    /// Get the contiguous faces as components
    pub fn components(&self) -> Vec<Vec<usize>> {
        unimplemented!();
    }

    /// Get the indices of the vertices shared between two faces
    pub fn shared_vertices(&self, i: usize, j: usize) -> Vec<usize> {
        unimplemented!();
    }

    /// Orient the mesh
    pub fn orient(&mut self) {
        unimplemented!();
    }

    /// Zip any open edges. This may result in a non-manifold mesh.
    pub fn zip_edges(&mut self) -> Result<(), HeMeshError> {
        unimplemented!();
    }

    /// Get the half edge pairs whose incident faces form an angle greater
    /// than the threshold
    pub fn feature_edges(&self, threshold: f64) -> Vec<(usize, usize)> {
        unimplemented!();
    }

    /// Get the principal axes defining the dominant orthogonal coordinate
    /// system local to the mesh vertices.
    pub fn principal_axes(&self) -> Vec<Vector3> {
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

    /// Extract the subset of patches by index into a new mesh
    pub fn extract_patches(&self, patches: &[usize]) -> HeMesh {
        unimplemented!();
    }

    /// Extract the subset of patches by name into a new mesh
    pub fn extract_patch_names(&self, names: &[&str]) -> HeMesh {
        let mut index = HashSet::<&str>::new();
        let mut patches = Vec::<usize>::new();

        for name in names.iter() {
            index.insert(name);
        }

        for (i, patch) in self.patches.iter().enumerate() {
            if index.contains(patch.name()) {
                patches.push(i);
            }
        }

        self.extract_patches(&patches)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct HeVertex {
    origin: Vector3,
    half_edge: usize,
}

impl HeVertex {
    /// Get the origin
    pub fn origin(&self) -> Vector3 {
        self.origin
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

impl std::error::Error for HeMeshError {}

impl Into<std::io::Error> for HeMeshError {
    fn into(self) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::InvalidData, self.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn import_obj() {
        let path = "tests/fixtures/box.obj";
        let mesh = HeMesh::import_obj(&path).unwrap();

        assert_eq!(mesh.n_vertices(), 8);
        assert_eq!(mesh.n_faces(), 12);
        assert_eq!(mesh.n_half_edges(), 36);
        assert_eq!(mesh.n_patches(), 0);
    }

    #[test]
    fn import_obj_gzip() {
        let path = "tests/fixtures/box.obj.gz";
        let mesh = HeMesh::import_obj(&path).unwrap();

        assert_eq!(mesh.n_vertices(), 8);
        assert_eq!(mesh.n_faces(), 12);
        assert_eq!(mesh.n_half_edges(), 36);
        assert_eq!(mesh.n_patches(), 0);
    }

    #[test]
    fn import_obj_patches() {
        let path = "tests/fixtures/box.groups.obj";
        let mesh = HeMesh::import_obj(&path).unwrap();

        assert_eq!(mesh.n_patches(), 6);
        assert_eq!(mesh.faces[0].patch, Some(0));
        assert_eq!(mesh.faces[1].patch, Some(1));
        assert_eq!(mesh.faces[2].patch, Some(1));
        assert_eq!(mesh.faces[3].patch, Some(2));
        assert_eq!(mesh.faces[4].patch, Some(3));
        assert_eq!(mesh.faces[5].patch, Some(4));
        assert_eq!(mesh.faces[6].patch, Some(5));
    }

    #[test]
    fn import_obj_nonmanifold() {
        let path = "tests/fixtures/box.nonmanifold.obj";
        let result = HeMesh::import_obj(&path);

        assert!(result.is_err_and(|e| e.to_string() == "non-manifold mesh"));
    }
}
