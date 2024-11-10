use crate::geometry::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Vector3,
    radius: f64,
}

impl Sphere {
    /// Construct a Sphere from its center and radius
    pub fn new(center: Vector3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    /// Get the center
    pub fn center(&self) -> Vector3 {
        self.center
    }

    /// Get the radius
    pub fn radius(&self) -> f64 {
        self.radius
    }
}
