use crate::geometry::{Geometry, Triangle};

/// Compute the intersection of a Triangle/Triangle. For most cases, this
/// will return a line segment. In the case of coplanar triangles, this
/// may return a point, line segment, or a triangle.
pub fn intersection_triangle_triangle(_t0: &Triangle, _t1: &Triangle) -> Option<Geometry> {
    // TODO: implement
    unimplemented!();
}
