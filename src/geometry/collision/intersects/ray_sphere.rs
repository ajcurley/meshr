use crate::geometry::collision::Intersects;
use crate::geometry::{Ray, Sphere, Vector3};

/// Check for a Ray/Sphere spatial intersection
fn intersects_ray_sphere(r: &Ray, s: &Sphere) -> bool {
    let l = s.center() - r.origin();
    let ld = Vector3::dot(&l, &r.direction());
    let ll = Vector3::dot(&l, &l);
    let rr = s.radius() * s.radius();

    if (ld < 0.) && (ll > rr) {
        return false;
    }

    (ll - ld * ld) <= rr
}

impl Intersects<Ray> for Sphere {
    fn intersects(&self, r: &Ray) -> bool {
        intersects_ray_sphere(r, self)
    }
}

impl Intersects<Sphere> for Ray {
    fn intersects(&self, s: &Sphere) -> bool {
        intersects_ray_sphere(self, s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hit() {
        let o = Vector3::new(-1., 0., 0.);
        let d = Vector3::new(1., 0., 0.);
        let r = Ray::new(o, d);
        let c = Vector3::new(0., 0., 0.);
        let s = Sphere::new(c, 0.1);

        assert!(intersects_ray_sphere(&r, &s));
    }

    #[test]
    fn miss() {
        let o = Vector3::new(-1., 0., 0.);
        let d = Vector3::new(-1., 0., 0.);
        let r = Ray::new(o, d);
        let c = Vector3::new(0., 0., 0.);
        let s = Sphere::new(c, 0.1);

        assert!(!intersects_ray_sphere(&r, &s));
    }
}
