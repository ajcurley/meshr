use crate::geometry::{Sphere, Vector3};

/// Check for a Sphere/Vector3 spatial intersection
pub fn intersects_sphere_vector3(s: &Sphere, v: &Vector3) -> bool {
    (*v - s.center()).mag() <= s.radius() * s.radius()
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
