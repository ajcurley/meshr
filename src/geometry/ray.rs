use crate::geometry::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    /// Construct a Ray from its origin and direction
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    /// Get the origin
    pub fn origin(&self) -> Vector3 {
        self.origin
    }

    /// Get the direction
    pub fn direction(&self) -> Vector3 {
        self.direction
    }
}
