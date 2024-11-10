use crate::geometry::collision::Intersects;
use crate::geometry::{Aabb, Vector3};

/// Check for an AABB/Vector3 spatial intersection
fn intersects_aabb_vector3(a: &Aabb, v: &Vector3) -> bool {
    let min = a.min();
    let max = a.max();

    v[0] >= min[0]
        && v[0] <= max[0]
        && v[1] >= min[1]
        && v[1] <= max[1]
        && v[2] >= min[2]
        && v[2] <= max[2]
}

impl Intersects<Aabb> for Vector3 {
    fn intersects(&self, a: &Aabb) -> bool {
        intersects_aabb_vector3(a, self)
    }
}

impl Intersects<Vector3> for Aabb {
    fn intersects(&self, v: &Vector3) -> bool {
        intersects_aabb_vector3(self, v)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hit() {
        let v = Vector3::new(0.4, 0.2, 0.1);
        let a = Aabb::unit();

        assert!(intersects_aabb_vector3(&a, &v));
    }

    #[test]
    fn miss_x_min() {
        let v = Vector3::new(-0.6, 0.2, 0.1);
        let a = Aabb::unit();

        assert!(!intersects_aabb_vector3(&a, &v));
    }

    #[test]
    fn miss_x_max() {
        let v = Vector3::new(0.6, 0.2, 0.1);
        let a = Aabb::unit();

        assert!(!intersects_aabb_vector3(&a, &v));
    }

    #[test]
    fn miss_y_min() {
        let v = Vector3::new(0.4, -0.6, 0.1);
        let a = Aabb::unit();

        assert!(!intersects_aabb_vector3(&a, &v));
    }

    #[test]
    fn miss_y_max() {
        let v = Vector3::new(0.4, 0.6, 0.1);
        let a = Aabb::unit();

        assert!(!intersects_aabb_vector3(&a, &v));
    }

    #[test]
    fn miss_z_min() {
        let v = Vector3::new(0.4, 0.2, -0.6);
        let a = Aabb::unit();

        assert!(!intersects_aabb_vector3(&a, &v));
    }

    #[test]
    fn miss_z_max() {
        let v = Vector3::new(0.4, 0.2, 0.6);
        let a = Aabb::unit();

        assert!(!intersects_aabb_vector3(&a, &v));
    }
}
