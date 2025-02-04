use crate::geometry::{Aabb, Sphere};

/// Check for an AABB/Sphere spatial intersection
pub fn intersects_aabb_sphere(a: &Aabb, s: &Sphere) -> bool {
    let center = s.center();
    let radius = s.radius();
    let min = a.min();
    let max = a.max();

    let mut d = 0.;

    for i in 0..3 {
        if center[i] < min[i] {
            let t = center[i] - min[i];
            d += t * t;
        } else if center[i] > max[i] {
            let t = center[i] - max[i];
            d += t * t;
        }
    }

    d <= radius * radius
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::geometry::Vector3;

    #[test]
    fn hit_overlap_full() {
        let c = Vector3::new(0.4, 0.1, 0.2);
        let s = Sphere::new(c, 0.05);
        let a = Aabb::unit();

        assert!(intersects_aabb_sphere(&a, &s));
    }

    #[test]
    fn hit_overlap_partial() {
        let c = Vector3::new(0.6, 0.1, 0.2);
        let s = Sphere::new(c, 0.2);
        let a = Aabb::unit();

        assert!(intersects_aabb_sphere(&a, &s));
    }

    #[test]
    fn miss_x_min() {
        let c = Vector3::new(-1., 0., 0.);
        let s = Sphere::new(c, 0.1);
        let a = Aabb::unit();

        assert!(!intersects_aabb_sphere(&a, &s));
    }

    #[test]
    fn miss_x_max() {
        let c = Vector3::new(1., 0., 0.);
        let s = Sphere::new(c, 0.1);
        let a = Aabb::unit();

        assert!(!intersects_aabb_sphere(&a, &s));
    }

    #[test]
    fn miss_y_min() {
        let c = Vector3::new(0., -1., 0.);
        let s = Sphere::new(c, 0.1);
        let a = Aabb::unit();

        assert!(!intersects_aabb_sphere(&a, &s));
    }

    #[test]
    fn miss_y_max() {
        let c = Vector3::new(0., 1., 0.);
        let s = Sphere::new(c, 0.1);
        let a = Aabb::unit();

        assert!(!intersects_aabb_sphere(&a, &s));
    }

    #[test]
    fn miss_z_min() {
        let c = Vector3::new(0., 0., -1.);
        let s = Sphere::new(c, 0.1);
        let a = Aabb::unit();

        assert!(!intersects_aabb_sphere(&a, &s));
    }

    #[test]
    fn miss_z_max() {
        let c = Vector3::new(0., 0., 1.);
        let s = Sphere::new(c, 0.1);
        let a = Aabb::unit();

        assert!(!intersects_aabb_sphere(&a, &s));
    }
}
