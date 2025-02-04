use crate::geometry::collision;
use crate::geometry::{Intersects, Vector3};

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

    /// Construct an Aabb from its min/max bounds
    pub fn from_bounds(min: Vector3, max: Vector3) -> Aabb {
        let center = (min + max) * 0.5;
        let halfsize = (max - min) * 0.5;
        Aabb::new(center, halfsize)
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

    /// Get the octant AABB using Morton encoding (Z-order)
    /// to identify the octant. 0 is the front/lower/left
    /// octant and 7 is the back/upper/right octant.
    pub fn octant(&self, octant: usize) -> Aabb {
        if octant >= 8 {
            panic!("invalid octant: {}", octant);
        }

        let h = self.halfsize * 0.5;
        let dx = if (octant & 4) == 0 { -h[0] } else { h[0] };
        let dy = if (octant & 2) == 0 { -h[1] } else { h[1] };
        let dz = if (octant & 1) == 0 { -h[2] } else { h[2] };
        let center = self.center + Vector3::new(dx, dy, dz);

        Aabb::new(center, h)
    }
}

impl Intersects<Vector3> for Aabb {
    fn intersects(&self, other: &Vector3) -> bool {
        collision::intersects::intersects_aabb_vector3(self, other)
    }
}
