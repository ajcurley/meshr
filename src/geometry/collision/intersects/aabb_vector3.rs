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
