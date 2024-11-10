use crate::geometry::collision::Intersects;
use crate::geometry::{Sphere, Vector3};

/// Check for a Sphere/Sphere spatial intersection
fn intersects_sphere_sphere(a: &Sphere, b: &Sphere) -> bool {
    let d = a.center() - b.center();
    let r = a.radius() + b.radius();
    Vector3::dot(&d, &d) <= r * r
}

impl Intersects<Sphere> for Sphere {
    fn intersects(&self, s: &Sphere) -> bool {
        intersects_sphere_sphere(self, s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hit_overlap_full() {
        let o = Vector3::new(0., 0., 0.);
        let a = Sphere::new(o, 1.);

        let o = Vector3::new(0.1, 0.1, 0.1);
        let b = Sphere::new(o, 0.2);

        assert!(intersects_sphere_sphere(&a, &b));
    }

    #[test]
    fn hit_overlap_partial() {
        let o = Vector3::new(0., 0., 0.);
        let a = Sphere::new(o, 1.);

        let o = Vector3::new(0.1, 0.1, 0.1);
        let b = Sphere::new(o, 0.2);

        assert!(intersects_sphere_sphere(&a, &b));
    }

    #[test]
    fn miss() {
        let o = Vector3::new(0., 0., 0.);
        let a = Sphere::new(o, 0.5);

        let o = Vector3::new(1., 1., 1.);
        let b = Sphere::new(o, 0.2);

        assert!(!intersects_sphere_sphere(&a, &b));
    }
}
