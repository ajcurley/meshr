use crate::geometry::collision;
use crate::geometry::{Aabb, Sphere, Triangle, Vector3};

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

impl crate::geometry::Intersects<Aabb> for Ray {
    fn intersects(&self, other: &Aabb) -> bool {
        collision::intersects::intersects_aabb_ray(other, self)
    }
}

impl crate::geometry::Intersects<Sphere> for Ray {
    fn intersects(&self, other: &Sphere) -> bool {
        collision::intersects::intersects_ray_sphere(self, other)
    }
}

impl crate::geometry::Intersects<Triangle> for Ray {
    fn intersects(&self, other: &Triangle) -> bool {
        collision::intersects::intersects_ray_triangle(self, other)
    }
}
