use std::collections::{HashMap, HashSet, VecDeque};

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

            for (k, &vertex_id) in vertices.iter().enumerate() {
                let prev = nh + ((k as i32 + nv as i32 - 1) % nv as i32) as usize;
                let next = nh + ((k as i32 + nv as i32 + 1) % nv as i32) as usize;

                let half_edge = HeHalfEdge {
                    origin: vertex_id,
                    face: face_id,
                    prev: prev,
                    next: next,
                    twin: None,
                };

                mesh.half_edges.push(half_edge);

                if let Some(vertex) = mesh.vertices.get_mut(vertex_id) {
                    vertex.half_edge = nh + k;
                }

                let ki = vertex_id;
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
        // TODO: implement
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

    /// Get the neighboring vertex indices to a vertex by index
    pub fn vertex_neighbors(&self, index: usize) -> Vec<usize> {
        HeVertexVertexIter::new(self, index).collect()
    }

    /// Get the faces using a vertex by index
    pub fn vertex_faces(&self, index: usize) -> Vec<usize> {
        HeVertexFaceIter::new(self, index).collect()
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

    /// Get the vertices used by a face by index
    pub fn face_vertices(&self, index: usize) -> Vec<usize> {
        HeFaceVertexIter::new(self, index).collect()
    }

    /// Get the neighboring face indices to a face by index
    pub fn face_neighbors(&self, index: usize) -> Vec<usize> {
        HeFaceFaceIter::new(self, index).collect()
    }

    /// Get the half edges defining the boundary of a face by index
    pub fn face_half_edges(&self, index: usize) -> Vec<usize> {
        HeFaceHalfEdgeIter::new(self, index).collect()
    }

    /// Flip a face by index. This reverses all half edges defining the boundary
    /// of the face to flip the orientation.
    fn flip_face(&mut self, index: usize) {
        let half_edge_ids = self.face_half_edges(index);
        let mut half_edges = Vec::<HeHalfEdge>::new();

        for &j in half_edge_ids.iter() {
            let half_edge = self.half_edges[j];
            let origin = self.half_edges[half_edge.next].origin;

            half_edges.push(HeHalfEdge {
                origin: origin,
                face: half_edge.face,
                prev: half_edge.next,
                next: half_edge.prev,
                twin: half_edge.twin,
            })
        }

        for (i, &j) in half_edge_ids.iter().enumerate() {
            self.half_edges[j] = half_edges[i];
        }
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

    /// Check if two faces are consistently oriented. If the two faces are
    /// not neighbors, this returns false.
    pub fn is_face_consistent(&self, _i: usize, _j: usize) -> bool {
        // TODO: implement
        unimplemented!();
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
        // TODO: implement
        unimplemented!();
    }

    /// Get the indices of the vertices shared between two faces
    pub fn shared_vertices(&self, _i: usize, _j: usize) -> Vec<usize> {
        // TODO: implement
        unimplemented!();
    }

    /// Orient the mesh
    pub fn orient(&mut self) {
        let mut oriented = vec![false; self.n_faces()];

        for component in self.components() {
            let mut queue = VecDeque::from([component[0]]);

            while let Some(current) = queue.pop_front() {
                if !oriented[current] {
                    oriented[current] = true;

                    for neighbor in self.face_neighbors(current) {
                        if !oriented[current] {
                            queue.push_back(neighbor);

                            if !self.is_face_consistent(current, neighbor) {
                                self.flip_face(neighbor);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Zip any open edges. This may result in a non-manifold mesh.
    pub fn zip_edges(&mut self) -> Result<(), HeMeshError> {
        // TODO: implement
        unimplemented!();
    }

    /// Get the half edge pairs whose incident faces form an angle greater
    /// than the threshold
    pub fn feature_edges(&self, _threshold: f64) -> Vec<(usize, usize)> {
        // TODO: implement
        unimplemented!();
    }

    /// Get the principal axes defining the dominant orthogonal coordinate
    /// system local to the mesh vertices.
    pub fn principal_axes(&self) -> Vec<Vector3> {
        // TODO: implement
        unimplemented!();
    }

    /// Merge naively with another mesh
    pub fn merge(&mut self, _other: &HeMesh) {
        // TODO: implement
        unimplemented!();
    }

    /// Extract the subset of faces into a new mesh. This is not efficient and should
    /// only be used when explicitly necessary.
    pub fn extract_faces(&self, _faces: &[usize]) -> HeMesh {
        // TODO: implement
        unimplemented!()
    }

    /// Extract the subset of patches by index into a new mesh
    pub fn extract_patches(&self, patches: &[usize]) -> HeMesh {
        let mut index = HashSet::<usize>::new();
        let mut faces = Vec::<usize>::new();

        for patch in patches.iter() {
            index.insert(*patch);
        }

        for (i, face) in self.faces.iter().enumerate() {
            if let Some(patch) = face.patch {
                if index.contains(&patch) {
                    faces.push(i);
                }
            }
        }

        self.extract_faces(&faces)
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
pub struct HeVertexOHalfEdgeIter<'a> {
    mesh: &'a HeMesh,
    curr: usize,
    init: usize,
    count: usize,
}

impl<'a> HeVertexOHalfEdgeIter<'a> {
    pub fn new(mesh: &'a HeMesh, vertex: usize) -> HeVertexOHalfEdgeIter<'a> {
        HeVertexOHalfEdgeIter {
            mesh: mesh,
            curr: mesh.vertices[vertex].half_edge,
            init: mesh.vertices[vertex].half_edge,
            count: 0,
        }
    }
}

impl<'a> Iterator for HeVertexOHalfEdgeIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count != 0 && self.curr == self.init {
            return None;
        }

        let curr = self.curr;
        let prev = self.mesh.half_edges[curr].prev;

        if let Some(twin) = self.mesh.half_edges[prev].twin {
            if self.mesh.half_edges[twin].origin != self.mesh.half_edges[self.init].origin {
                panic!("mesh must be oriented");
            }

            self.curr = twin;
            self.count += 1;
            return Some(curr);
        }

        panic!("mesh must be closed");
    }
}

#[derive(Debug, Clone)]
pub struct HeVertexIHalfEdgeIter<'a> {
    mesh: &'a HeMesh,
    iter: HeVertexOHalfEdgeIter<'a>,
}

impl<'a> HeVertexIHalfEdgeIter<'a> {
    pub fn new(mesh: &'a HeMesh, vertex: usize) -> HeVertexIHalfEdgeIter<'a> {
        HeVertexIHalfEdgeIter {
            mesh: mesh,
            iter: HeVertexOHalfEdgeIter::new(mesh, vertex),
        }
    }
}

impl<'a> Iterator for HeVertexIHalfEdgeIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr) = self.iter.next() {
            return self.mesh.half_edges[curr].twin;
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct HeVertexVertexIter<'a> {
    mesh: &'a HeMesh,
    iter: HeVertexOHalfEdgeIter<'a>,
}

impl<'a> HeVertexVertexIter<'a> {
    pub fn new(mesh: &'a HeMesh, vertex: usize) -> HeVertexVertexIter<'a> {
        HeVertexVertexIter {
            mesh: mesh,
            iter: HeVertexOHalfEdgeIter::new(mesh, vertex),
        }
    }
}

impl<'a> Iterator for HeVertexVertexIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr) = self.iter.next() {
            let next = self.mesh.half_edges[curr].next;
            return Some(self.mesh.half_edges[next].origin);
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct HeVertexFaceIter<'a> {
    mesh: &'a HeMesh,
    iter: HeVertexOHalfEdgeIter<'a>,
}

impl<'a> HeVertexFaceIter<'a> {
    pub fn new(mesh: &'a HeMesh, vertex: usize) -> HeVertexFaceIter<'a> {
        HeVertexFaceIter {
            mesh: mesh,
            iter: HeVertexOHalfEdgeIter::new(mesh, vertex),
        }
    }
}

impl<'a> Iterator for HeVertexFaceIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr) = self.iter.next() {
            return Some(self.mesh.half_edges[curr].face);
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct HeFaceHalfEdgeIter<'a> {
    mesh: &'a HeMesh,
    curr: usize,
    init: usize,
    count: usize,
}

impl<'a> HeFaceHalfEdgeIter<'a> {
    pub fn new(mesh: &'a HeMesh, face: usize) -> HeFaceHalfEdgeIter {
        HeFaceHalfEdgeIter {
            mesh: mesh,
            init: mesh.faces[face].half_edge,
            curr: mesh.faces[face].half_edge,
            count: 0,
        }
    }
}

impl<'a> Iterator for HeFaceHalfEdgeIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count != 0 && self.curr == self.init {
            return None;
        }

        let curr = self.curr;
        self.curr = self.mesh.half_edges[self.curr].next;
        self.count += 1;

        Some(curr)
    }
}

#[derive(Debug, Clone)]
pub struct HeFaceVertexIter<'a> {
    mesh: &'a HeMesh,
    iter: HeFaceHalfEdgeIter<'a>,
}

impl<'a> HeFaceVertexIter<'a> {
    pub fn new(mesh: &'a HeMesh, face: usize) -> HeFaceVertexIter<'a> {
        HeFaceVertexIter {
            mesh: mesh,
            iter: HeFaceHalfEdgeIter::new(mesh, face),
        }
    }
}

impl<'a> Iterator for HeFaceVertexIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.iter.next() {
            return Some(self.mesh.half_edges[index].origin);
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct HeFaceFaceIter<'a> {
    mesh: &'a HeMesh,
    iter: HeFaceHalfEdgeIter<'a>,
}

impl<'a> HeFaceFaceIter<'a> {
    pub fn new(mesh: &'a HeMesh, face: usize) -> HeFaceFaceIter<'a> {
        HeFaceFaceIter {
            mesh: mesh,
            iter: HeFaceHalfEdgeIter::new(mesh, face),
        }
    }
}

impl<'a> Iterator for HeFaceFaceIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(curr) = self.iter.next() {
            if let Some(twin) = self.mesh.half_edges[curr].twin {
                return Some(self.mesh.half_edges[twin].face);
            }
        }

        None
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

    #[test]
    fn face_half_edge_iter() {
        let path = "tests/fixtures/box.obj";
        let mesh = HeMesh::import_obj(&path).unwrap();
        let mut iter = HeFaceHalfEdgeIter::new(&mesh, 0);

        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn face_vertex_iter() {
        let path = "tests/fixtures/box.obj";
        let mesh = HeMesh::import_obj(&path).unwrap();
        let mut iter = HeFaceVertexIter::new(&mesh, 0);

        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn face_face_iter() {
        let path = "tests/fixtures/box.obj";
        let mesh = HeMesh::import_obj(&path).unwrap();
        let mut iter = HeFaceFaceIter::new(&mesh, 0);

        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(8));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn vertex_outgoing_half_edge_iter() {
        let path = "tests/fixtures/box.obj";
        let mesh = HeMesh::import_obj(&path).unwrap();
        let mut iter = HeVertexOHalfEdgeIter::new(&mesh, 0);

        assert_eq!(iter.next(), Some(24));
        assert_eq!(iter.next(), Some(12));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), None);
    }

    #[test]
    #[should_panic]
    fn vertex_outgoing_half_edge_iter_open() {
        let path = "tests/fixtures/box.open.obj";
        let mesh = HeMesh::import_obj(&path).unwrap();
        let mut iter = HeVertexOHalfEdgeIter::new(&mesh, 3);

        iter.next();
        iter.next();
    }

    #[test]
    #[should_panic]
    fn vertex_outgoing_half_edge_iter_inconsistent() {
        let path = "tests/fixtures/box.inconsistent.obj";
        let mesh = HeMesh::import_obj(&path).unwrap();
        let mut iter = HeVertexOHalfEdgeIter::new(&mesh, 1);

        iter.next();
        iter.next();
    }

    #[test]
    fn vertex_incoming_half_edge_iter() {
        let path = "tests/fixtures/box.obj";
        let mesh = HeMesh::import_obj(&path).unwrap();
        let mut iter = HeVertexIHalfEdgeIter::new(&mesh, 0);

        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(26));
        assert_eq!(iter.next(), Some(14));
        assert_eq!(iter.next(), None);
    }

    #[test]
    #[should_panic]
    fn vertex_incoming_half_edge_iter_open() {
        let path = "tests/fixtures/box.open.obj";
        let mesh = HeMesh::import_obj(&path).unwrap();
        let mut iter = HeVertexIHalfEdgeIter::new(&mesh, 3);

        iter.next();
        iter.next();
    }

    #[test]
    #[should_panic]
    fn vertex_incoming_half_edge_iter_inconsistent() {
        let path = "tests/fixtures/box.inconsistent.obj";
        let mesh = HeMesh::import_obj(&path).unwrap();
        let mut iter = HeVertexOHalfEdgeIter::new(&mesh, 1);

        iter.next();
        iter.next();
    }

    #[test]
    fn vertex_vertex_iter() {
        let path = "tests/fixtures/box.obj";
        let mesh = HeMesh::import_obj(&path).unwrap();
        let mut iter = HeVertexVertexIter::new(&mesh, 6);

        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), None);
    }

    #[test]
    #[should_panic]
    fn vertex_vertex_iter_open() {
        let path = "tests/fixtures/box.open.obj";
        let mesh = HeMesh::import_obj(&path).unwrap();
        let mut iter = HeVertexVertexIter::new(&mesh, 3);

        iter.next();
        iter.next();
    }

    #[test]
    #[should_panic]
    fn vertex_vertex_iter_inconsistent() {
        let path = "tests/fixtures/box.inconsistent.obj";
        let mesh = HeMesh::import_obj(&path).unwrap();
        let mut iter = HeVertexVertexIter::new(&mesh, 1);

        iter.next();
        iter.next();
    }

    #[test]
    fn vertex_face_iter() {
        let path = "tests/fixtures/box.obj";
        let mesh = HeMesh::import_obj(&path).unwrap();
        let mut iter = HeVertexFaceIter::new(&mesh, 6);

        assert_eq!(iter.next(), Some(9));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    #[should_panic]
    fn vertex_face_iter_open() {
        let path = "tests/fixtures/box.open.obj";
        let mesh = HeMesh::import_obj(&path).unwrap();
        let mut iter = HeVertexFaceIter::new(&mesh, 3);

        iter.next();
        iter.next();
    }

    #[test]
    #[should_panic]
    fn vertex_face_iter_inconsistent() {
        let path = "tests/fixtures/box.inconsistent.obj";
        let mesh = HeMesh::import_obj(&path).unwrap();
        let mut iter = HeVertexFaceIter::new(&mesh, 1);

        iter.next();
        iter.next();
    }

    #[test]
    fn flip_face() {
        let path = "tests/fixtures/box.obj";
        let mut mesh = HeMesh::import_obj(&path).unwrap();
        assert!(mesh.is_closed());
        assert!(mesh.is_consistent());

        let vertices = mesh.face_vertices(0);
        assert_eq!(vertices[0], 0);
        assert_eq!(vertices[1], 1);
        assert_eq!(vertices[2], 2);

        mesh.flip_face(0);
        assert!(mesh.is_closed());
        assert!(!mesh.is_consistent());

        let vertices = mesh.face_vertices(0);
        assert_eq!(vertices[0], 1);
        assert_eq!(vertices[1], 0);
        assert_eq!(vertices[2], 2);
    }
}
