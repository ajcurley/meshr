use crate::geometry::collision;
use crate::geometry::{Geometry, Triangle, Vector3};

#[derive(Debug, Copy, Clone)]
pub struct Line {
    p: Vector3,
    q: Vector3,
}

impl Line {
    /// Construct a Line from its endpoints
    pub fn new(p: Vector3, q: Vector3) -> Line {
        Line { p, q }
    }

    /// Get the vertices as a tuple
    pub fn vertices(&self) -> (Vector3, Vector3) {
        (self.p, self.q)
    }

    /// Get the p-component
    pub fn p(&self) -> Vector3 {
        self.p
    }

    /// Get the q-component
    pub fn q(&self) -> Vector3 {
        self.q
    }

    /// Compute the vector direction
    pub fn direction(&self) -> Vector3 {
        self.q - self.p
    }

    /// Get the length of the segment
    pub fn length(&self) -> f64 {
        self.direction().mag()
    }
}

impl std::ops::Index<usize> for Line {
    type Output = Vector3;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.p,
            1 => &self.q,
            _ => panic!("index out of range"),
        }
    }
}

impl std::ops::IndexMut<usize> for Line {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.p,
            1 => &mut self.q,
            _ => panic!("index out of range"),
        }
    }
}

impl crate::geometry::Intersection<Triangle> for Line {
    fn intersection(&self, other: &Triangle) -> Option<Geometry> {
        collision::intersection::intersection_line_triangle(self, other)
    }
}
