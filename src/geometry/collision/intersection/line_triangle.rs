use crate::geometry::{Geometry, Line, Triangle};

/// Compute the intersection of a Line/Triangle. For an out-of-plane line segment,
/// this will return a Point geometry and for a coplanar line segment, this will
/// return a line segment assuming an intersection.
pub fn intersection_line_triangle(_line: &Line, _triangle: &Triangle) -> Option<Geometry> {
    // TODO: implement
    unimplemented!();
}
