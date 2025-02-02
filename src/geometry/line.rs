use crate::geometry::Vector3;

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

    /// Get the p-component
    pub fn p(&self) -> Vector3 {
        self.p
    }

    /// Get the q-component
    pub fn q(&self) -> Vector3 {
        self.q
    }

    /// Compute the unit vector direction
    pub fn direction(&self) -> Vector3 {
        (self.q - self.p).unit()
    }

    /// Get the length of the segment
    pub fn length(&self) -> f64 {
        (self.q - self.p).mag()
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
