use crate::geometry::collision::Intersects;
use crate::geometry::Aabb;

/// Check for a spatial intersection between two Aabb
fn intersects_aabb_aabb(a: &Aabb, b: &Aabb) -> bool {
    let min_a = a.min();
    let max_a = a.max();
    let min_b = b.min();
    let max_b = b.max();

    min_a[0] <= max_b[0]
        && max_a[0] >= min_b[0]
        && min_a[1] <= max_b[1]
        && max_a[1] >= min_b[1]
        && min_a[2] <= max_b[2]
        && max_a[2] >= min_b[2]
}

impl Intersects<Aabb> for Aabb {
    fn intersects(&self, other: &Aabb) -> bool {
        intersects_aabb_aabb(self, other)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::geometry::Vector3;

    #[test]
    fn hit_overlap_full() {
        let center = Vector3::zeros();
        let halfsize = Vector3::new(0.1, 0.1, 0.1);
        let other = Aabb::new(center, halfsize);

        assert!(Aabb::unit().intersects(&other));
    }

    #[test]
    fn hit_overlap_partial() {
        let center = Vector3::new(0.4, 0.4, 0.4);
        let halfsize = Vector3::new(0.2, 0.2, 0.2);
        let other = Aabb::new(center, halfsize);

        assert!(Aabb::unit().intersects(&other));
    }

    #[test]
    fn miss_overlay_x_min_only() {
        let center = Vector3::new(-0.4, 1.2, 1.2);
        let halfsize = Vector3::new(0.2, 0.2, 0.2);
        let other = Aabb::new(center, halfsize);

        assert!(!Aabb::unit().intersects(&other));
    }

    #[test]
    fn miss_overlap_x_max_only() {
        let center = Vector3::new(0.4, 1.2, 1.2);
        let halfsize = Vector3::new(0.2, 0.2, 0.2);
        let other = Aabb::new(center, halfsize);

        assert!(!Aabb::unit().intersects(&other));
    }

    #[test]
    fn miss_overlay_y_min_only() {
        let center = Vector3::new(1.2, -0.4, 1.2);
        let halfsize = Vector3::new(0.2, 0.2, 0.2);
        let other = Aabb::new(center, halfsize);

        assert!(!Aabb::unit().intersects(&other));
    }

    #[test]
    fn miss_overlap_y_max_only() {
        let center = Vector3::new(1.2, 0.4, 1.2);
        let halfsize = Vector3::new(0.2, 0.2, 0.2);
        let other = Aabb::new(center, halfsize);

        assert!(!Aabb::unit().intersects(&other));
    }

    #[test]
    fn miss_overlay_z_min_only() {
        let center = Vector3::new(1.2, 1.2, -0.4);
        let halfsize = Vector3::new(0.2, 0.2, 0.2);
        let other = Aabb::new(center, halfsize);

        assert!(!Aabb::unit().intersects(&other));
    }

    #[test]
    fn miss_overlap_z_max_only() {
        let center = Vector3::new(1.2, 1.2, 0.4);
        let halfsize = Vector3::new(0.2, 0.2, 0.2);
        let other = Aabb::new(center, halfsize);

        assert!(!Aabb::unit().intersects(&other));
    }
}