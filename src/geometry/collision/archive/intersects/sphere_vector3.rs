use crate::geometry::collision::Intersects;
use crate::geometry::{Sphere, Vector3};

/// Check for a Sphere/Vector3 spatial intersection
fn intersects_sphere_vector3(s: &Sphere, v: &Vector3) -> bool {
    (*v - s.center()).mag() <= s.radius() * s.radius()
}

impl Intersects<Sphere> for Vector3 {
    fn intersects(&self, s: &Sphere) -> bool {
        intersects_sphere_vector3(s, self)
    }
}

impl Intersects<Vector3> for Sphere {
    fn intersects(&self, v: &Vector3) -> bool {
        intersects_sphere_vector3(self, v)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hit() {
        let v = Vector3::new(0.1, 0.2, 0.3);
        let c = Vector3::new(0., 0., 0.);
        let s = Sphere::new(c, 1.);

        assert!(intersects_sphere_vector3(&s, &v));
    }

    #[test]
    fn miss() {
        let v = Vector3::new(1., 1., 1.);
        let c = Vector3::new(0., 0., 0.);
        let s = Sphere::new(c, 0.1);

        assert!(!intersects_sphere_vector3(&s, &v));
    }
}
