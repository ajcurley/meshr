use crate::geometry::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    p: Vector3,
    q: Vector3,
    r: Vector3,
}

impl Triangle {
    /// Construct a Triangle from its vertices
    pub fn new(p: Vector3, q: Vector3, r: Vector3) -> Triangle {
        Triangle { p, q, r }
    }
}

impl std::ops::Index<usize> for Triangle {
    type Output = Vector3;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.p,
            1 => &self.q,
            2 => &self.r,
            _ => panic!("index out of range"),
        }
    }
}
