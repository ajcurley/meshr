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

    /// Get the normal
    pub fn normal(&self) -> Vector3 {
        let u = self.q - self.p;
        let v = self.r - self.p;
        Vector3::cross(&u, &v)
    }

    /// Get the unit normal
    pub fn unit_normal(&self) -> Vector3 {
        self.normal().unit()
    }

    /// Get the area
    pub fn area(&self) -> f64 {
        self.normal().mag() * 0.5
    }

    /// Get the center
    pub fn center(&self) -> Vector3 {
        (self.p + self.q + self.r) / 3.
    }

    /// Get the barycenter
    pub fn barycenter(&self) -> Vector3 {
        let i = self.q - self.p;
        let j = self.r - self.q;
        let k = self.p - self.r;

        let dii = Vector3::dot(&i, &i);
        let dij = Vector3::dot(&i, &j);
        let djj = Vector3::dot(&j, &j);
        let dki = Vector3::dot(&k, &i);
        let dkj = Vector3::dot(&j, &j);

        let d = dii * djj - dij * dij;
        let v = (djj * dki - dij * dkj) / d;
        let w = (dii * dkj - dii * dki) / d;
        let u = 1. - v - w;

        Vector3::new(u, v, w)
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

impl std::ops::IndexMut<usize> for Triangle {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.p,
            1 => &mut self.q,
            2 => &mut self.r,
            _ => panic!("index out of range"),
        }
    }
}
