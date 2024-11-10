use crate::geometry::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Aabb {
    center: Vector3,
    halfsize: Vector3,
}

impl Aabb {
    /// Construct an Aabb from its center and halfsize
    pub fn new(center: Vector3, halfsize: Vector3) -> Aabb {
        Aabb { center, halfsize }
    }

    /// Construct a unit Aabb
    pub fn unit() -> Aabb {
        let center = Vector3::zeros();
        let halfsize = Vector3::new(0.5, 0.5, 0.5);
        Aabb::new(center, halfsize)
    }

    /// Get the center
    pub fn center(&self) -> Vector3 {
        self.center
    }

    /// Get the halfsize
    pub fn halfsize(&self) -> Vector3 {
        self.halfsize
    }

    /// Get the min bound
    pub fn min(&self) -> Vector3 {
        self.center - self.halfsize
    }

    /// Get the max bound
    pub fn max(&self) -> Vector3 {
        self.center + self.halfsize
    }
}
