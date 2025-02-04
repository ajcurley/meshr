use crate::geometry::collision;
use crate::geometry::{Aabb, Intersects, Ray, Vector3};

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

impl Intersects<Aabb> for Sphere {
    fn intersects(&self, other: &Aabb) -> bool {
        collision::intersects::intersects_aabb_sphere(other, self)
    }
}

impl Intersects<Ray> for Sphere {
    fn intersects(&self, other: &Ray) -> bool {
        collision::intersects::intersects_ray_sphere(other, self)
    }
}

impl Intersects<Sphere> for Sphere {
    fn intersects(&self, other: &Sphere) -> bool {
        collision::intersects::intersects_sphere_sphere(self, other)
    }
}

impl Intersects<Vector3> for Sphere {
    fn intersects(&self, other: &Vector3) -> bool {
        collision::intersects::intersects_sphere_vector3(self, other)
    }
}
