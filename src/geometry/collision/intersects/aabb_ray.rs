use crate::geometry::{Aabb, Ray};

/// Check for an AABB/Ray spatial intersection
pub fn intersects_aabb_ray(a: &Aabb, r: &Ray) -> bool {
    let origin = r.origin();
    let inv = r.direction().inv();
    let min = a.min();
    let max = a.max();

    let tx0 = (min[0] - origin[0]) * inv[0];
    let tx1 = (max[0] - origin[0]) * inv[0];
    let tmin = tx0.min(tx1);
    let tmax = tx0.max(tx1);

    let ty0 = (min[1] - origin[1]) * inv[1];
    let ty1 = (max[1] - origin[1]) * inv[1];
    let tmin = tmin.max(ty0.min(ty1));
    let tmax = tmax.min(ty0.max(ty1));

    let tz0 = (min[2] - origin[2]) * inv[2];
    let tz1 = (max[2] - origin[2]) * inv[2];
    let tmin = tmin.max(tz0.min(tz1));
    let tmax = tmax.min(tz0.max(tz1));

    tmax >= tmin.max(0.)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::geometry::Vector3;

    #[test]
    fn hit_diagonal() {
        let o = Vector3::new(-1., -1., -1.);
        let d = Vector3::new(1., 1., 1.);
        let r = Ray::new(o, d);
        let a = Aabb::unit();

        assert!(intersects_aabb_ray(&a, &r));
    }

    #[test]
    fn hit_x_aligned() {
        let o = Vector3::new(-1., 0., 0.);
        let d = Vector3::new(1., 0., 0.);
        let r = Ray::new(o, d);
        let a = Aabb::unit();

        assert!(intersects_aabb_ray(&a, &r));
    }

    #[test]
    fn hit_y_aligned() {
        let o = Vector3::new(0., -1., 0.);
        let d = Vector3::new(0., 1., 0.);
        let r = Ray::new(o, d);
        let a = Aabb::unit();

        assert!(intersects_aabb_ray(&a, &r));
    }

    #[test]
    fn hit_z_aligned() {
        let o = Vector3::new(0., 0., -1.);
        let d = Vector3::new(0., 0., 1.);
        let r = Ray::new(o, d);
        let a = Aabb::unit();

        assert!(intersects_aabb_ray(&a, &r));
    }

    #[test]
    fn miss_diagonal() {
        let o = Vector3::new(1., 1., 1.);
        let d = Vector3::new(1., 1., 1.);
        let r = Ray::new(o, d);
        let a = Aabb::unit();

        assert!(!intersects_aabb_ray(&a, &r));
    }

    #[test]
    fn miss_x_aligned() {
        let o = Vector3::new(1., 0., 0.);
        let d = Vector3::new(0., 0., -1.);
        let r = Ray::new(o, d);
        let a = Aabb::unit();

        assert!(!intersects_aabb_ray(&a, &r));
    }

    #[test]
    fn miss_y_aligned() {
        let o = Vector3::new(0., 1., 0.);
        let d = Vector3::new(0., 0., -1.);
        let r = Ray::new(o, d);
        let a = Aabb::unit();

        assert!(!intersects_aabb_ray(&a, &r));
    }

    #[test]
    fn miss_z_aligned() {
        let o = Vector3::new(0., 0., 1.);
        let d = Vector3::new(-1., 0., 0.);
        let r = Ray::new(o, d);
        let a = Aabb::unit();

        assert!(!intersects_aabb_ray(&a, &r));
    }
}
