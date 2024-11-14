use crate::geometry::Vector3;
use crate::mesh::{ObjReader, PolygonSoupMesh};

#[derive(Debug, Clone)]
pub struct HeMesh {
    vertices: Vec<HeVertex>,
    faces: Vec<HeFace>,
    half_edges: Vec<HeHalfEdge>,
    patches: Vec<HePatch>,
}

impl HeMesh {
    /// Construct a half edge mesh from a polygon soup mesh
    pub fn from_polygon_soup(soup: &PolygonSoupMesh) -> HeMesh {
        unimplemented!();
    }

    /// Import a half edge mesh from an OBJ file
    pub fn import_obj(path: &str) -> std::io::Result<HeMesh> {
        let soup = ObjReader::new(&path).read()?;
        let mesh = HeMesh::from_polygon_soup(&soup);
        Ok(mesh)
    }

    /// Export a half edge mesh to an OBJ file
    pub fn export_obj(path: &str) {
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
    pub fn merge(&mut self, other: &HeMesh) {
        unimplemented!();
    }

    /// Extract the subset of faces into a new mesh
    pub fn extract_face(&self, faces: &[usize]) -> HeMesh {
        unimplemented!();
    }

    /// Extract the subset of patches by name into a new mesh
    pub fn extract_patches(&self, patches: &[&str]) -> HeMesh {
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
