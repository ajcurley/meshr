use crate::geometry::{Aabb, Vector3};

/// Check for an AABB/Vector3 spatial intersection
pub fn intersects_aabb_vector3(a: &Aabb, v: &Vector3) -> bool {
    let halfsize = a.halfsize();
    let center = a.center();

    (center[0] - halfsize[0]) <= v[0]
        && (center[0] + halfsize[0]) >= v[0]
        && (center[1] - halfsize[1]) <= v[1]
        && (center[1] + halfsize[1]) >= v[1]
        && (center[2] - halfsize[2]) <= v[2]
        && (center[2] + halfsize[2]) >= v[2]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn intersects_contained() {
        let aabb = Aabb::unit();
        let point = Vector3::new(-0.1, 0.4, 0.2);

        let result = intersects_aabb_vector3(&aabb, &point);

        assert!(result);
    }

    #[test]
    fn intersects_on_edge() {
        let aabb = Aabb::unit();
        let point = Vector3::new(0.5, 0.5, 0.2);

        let result = intersects_aabb_vector3(&aabb, &point);

        assert!(result);
    }

    #[test]
    fn intersects_on_face() {
        let aabb = Aabb::unit();
        let point = Vector3::new(0.2, 0.4, 0.5);

        let result = intersects_aabb_vector3(&aabb, &point);

        assert!(result);
    }

    #[test]
    fn miss() {
        let aabb = Aabb::unit();
        let point = Vector3::new(2., 2., 2.);

        let result = intersects_aabb_vector3(&aabb, &point);

        assert!(!result);
    }
}
