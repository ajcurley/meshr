use crate::geometry::Vector3;

#[derive(Debug, Clone, Default)]
pub struct PolygonSoup {
    vertices: Vec<Vector3>,
    face_offsets: Vec<usize>,
    face_vertices: Vec<usize>,
    face_patches: Vec<Option<usize>>,
    patches: Vec<String>,
}

impl PolygonSoup {
    /// Construct an empty Polygon Soup mesh
    pub fn new() -> PolygonSoup {
        PolygonSoup::default()
    }

    /// Get the number of vertices
    pub fn n_vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Get a vertex
    pub fn vertex(&self, index: usize) -> Vector3 {
        self.vertices[index]
    }

    /// Insert a vertex
    pub fn insert_vertex(&mut self, position: Vector3) {
        self.vertices.push(position);
    }

    /// Get the number of faces
    pub fn n_faces(&self) -> usize {
        self.face_offsets.len()
    }

    /// Get a face definition
    pub fn face(&self, index: usize) -> (&[usize], Option<usize>) {
        let patch = self.face_patches[index];
        let start = self.face_offsets[index];

        if index < self.n_faces() - 1 {
            let end = self.face_offsets[index + 1];
            return (&self.face_vertices[start..end], patch);
        }

        (&self.face_vertices[start..], patch)
    }

    /// Insert a face
    pub fn insert_face(&mut self, vertices: &[usize], patch: Option<usize>) {
        let offset = self.face_vertices.len();
        self.face_offsets.push(offset);
        self.face_vertices.extend(vertices);
        self.face_patches.push(patch);
    }

    /// Get the number of patches
    pub fn n_patches(&self) -> usize {
        self.patches.len()
    }

    /// Get a borrowed reference to a patch
    pub fn patch(&self, index: usize) -> &str {
        &self.patches[index]
    }

    /// Insert a patch
    pub fn insert_patch(&mut self, name: &str) {
        self.patches.push(name.to_string());
    }
}
