use crate::geometry::collision::Intersects;
use crate::geometry::{Ray, Triangle, Vector3};

/// Geometric tolerance for an intersection
const EPSILON: f64 = 1e-8;

/// Check for a Ray/Triangle spatial intersection
fn intersects_ray_triangle(r: &Ray, t: &Triangle) -> bool {
    let origin = r.origin();
    let direction = r.direction();

    let e0 = t[1] - t[0];
    let e1 = t[2] - t[0];

    let p = Vector3::cross(&direction, &e1);
    let d = Vector3::dot(&e0, &p);

    if d < EPSILON {
        return false;
    }

    let d_inv = 1. / d;
    let s = origin - t[0];
    let u = d_inv * Vector3::dot(&s, &p);

    if u < 0. || u > 1. {
        return false;
    }

    let q = Vector3::cross(&s, &e0);
    let v = d_inv * Vector3::dot(&direction, &q);

    if v < 0. || u + v > 1. {
        return false;
    }

    (d_inv * Vector3::dot(&e1, &q)) > EPSILON
}

impl Intersects<Ray> for Triangle {
    fn intersects(&self, r: &Ray) -> bool {
        intersects_ray_triangle(r, self)
    }
}

impl Intersects<Triangle> for Ray {
    fn intersects(&self, t: &Triangle) -> bool {
        intersects_ray_triangle(self, t)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hit() {
        let o = Vector3::new(0.5, 0.5, 0.);
        let d = Vector3::new(0., 0., 1.);
        let r = Ray::new(o, d);

        let a = Vector3::new(0., 0., 1.);
        let b = Vector3::new(0., 1., 1.);
        let c = Vector3::new(1., 0., 1.);
        let t = Triangle::new(a, b, c);

        assert!(intersects_ray_triangle(&r, &t));
    }

    #[test]
    fn miss_culled() {
        let o = Vector3::new(0.5, 0.5, 0.);
        let d = Vector3::new(0., 0., 1.);
        let r = Ray::new(o, d);

        let a = Vector3::new(0., 0., 1.);
        let b = Vector3::new(1., 0., 1.);
        let c = Vector3::new(0., 1., 1.);
        let t = Triangle::new(a, b, c);

        assert!(!intersects_ray_triangle(&r, &t));
    }

    #[test]
    fn miss() {
        let o = Vector3::new(2., 2., 0.);
        let d = Vector3::new(0., 0., 1.);
        let r = Ray::new(o, d);

        let a = Vector3::new(0., 0., 1.);
        let b = Vector3::new(0., 1., 1.);
        let c = Vector3::new(1., 0., 1.);
        let t = Triangle::new(a, b, c);

        assert!(!intersects_ray_triangle(&r, &t));
    }
}
