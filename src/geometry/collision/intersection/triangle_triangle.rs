use crate::geometry::collision::Intersection;
use crate::geometry::{Line, Triangle, Vector3};

/// Compute the intersection line between two triangles. If the intersection
/// represents a point, the line will be comprised of equal points.
pub fn intersection_triangle_triangle(t0: &Triangle, t1: &Triangle) -> Option<Line> {
    // Check for coplanrity
    // Overlap test using SAT
    // Normals/cross products test
    // Edge/edge intersection

    // Test for coplanar triangles
    if Triangle::is_coplanar(t0, t1) {
        return None;
    }

    // Test the triangle normals and cross products with the edges for overlap using
    // the Separating Axis Theorem.
    let n0 = t0.normal();
    let n1 = t1.normal();

    if n0.mag() > 0. && !test_overlap(n0, &t0, &t1) {
        return None;
    }

    if n1.mag() > 0. && !test_overlap(n1, &t0, &t1) {
        return None;
    }

    for edge in t0.edges().iter() {
        let axis = Vector3::cross(&edge.direction(), &n1);

        if axis.mag() > 0. && !test_overlap(axis, &t0, &t1) {
            return None;
        }
    }

    for edge in t1.edges().iter() {
        let axis = Vector3::cross(&edge.direction(), &n0);

        if axis.mag() > 0. && !test_overlap(axis, &t0, &t1) {
            return None;
        }
    }

    // Since the overlap tests pass, it is known that there is an intersection or at
    // least a significant overlap. Compute the edge/triangle intersections.
    let mut points = vec![];

    for edge in t0.edges().iter() {
        if let Some(point) = edge.intersection(t1) {
            points.push(point);
        }
    }

    for edge in t1.edges().iter() {
        if let Some(point) = edge.intersection(t0) {
            points.push(point);
        }
    }

    match points.len() {
        1 => Some(Line::new(points[0], points[0])),
        2 => Some(Line::new(points[0], points[1])),
        _ => None,
    }
}

/// Test for overlap between the two triangles along an axis
fn test_overlap(axis: Vector3, t0: &Triangle, t1: &Triangle) -> bool {
    let mut p0_min = std::f64::INFINITY;
    let mut p0_max = std::f64::NEG_INFINITY;
    let mut p1_min = std::f64::INFINITY;
    let mut p1_max = std::f64::NEG_INFINITY;

    for i in 0..3 {
        let d0 = Vector3::dot(&t0[i], &axis);
        p0_min = p0_min.min(d0);
        p0_max = p0_max.max(d0);

        let d1 = Vector3::dot(&t1[i], &axis);
        p1_min = p1_min.min(d1);
        p1_max = p1_max.max(d1);
    }

    p0_max >= p1_min && p1_max >= p0_min
}

impl Intersection<Triangle> for Triangle {
    type Output = Line;

    fn intersection(&self, triangle: &Triangle) -> Option<Self::Output> {
        intersection_triangle_triangle(self, triangle)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point_to_point() {
        let a = Vector3::new(0., 0., 0.);
        let b = Vector3::new(1., 0., 0.);
        let c = Vector3::new(0., 1., 0.);
        let t0 = Triangle::new(a, b, c);

        let d = Vector3::new(0., 0., 0.);
        let e = Vector3::new(0., 0., 1.);
        let f = Vector3::new(0., 1., 1.);
        let t1 = Triangle::new(d, e, f);

        let result = intersection_triangle_triangle(&t0, &t1).unwrap();

        assert_eq!(result[0], Vector3::new(0., 0., 0.));
        assert_eq!(result[1], Vector3::new(0., 0., 0.));
    }

    #[test]
    fn test_point_to_plane() {
        let a = Vector3::new(0., 0., 0.);
        let b = Vector3::new(1., 0., 0.);
        let c = Vector3::new(0., 1., 0.);
        let t0 = Triangle::new(a, b, c);

        let d = Vector3::new(0.1, 0.1, 0.);
        let e = Vector3::new(0., 0., 1.);
        let f = Vector3::new(0., 1., 1.);
        let t1 = Triangle::new(d, e, f);

        let result = intersection_triangle_triangle(&t0, &t1).unwrap();

        assert_eq!(result[0], Vector3::new(0.1, 0.1, 0.));
        assert_eq!(result[1], Vector3::new(0.1, 0.1, 0.));
    }

    #[test]
    fn test_single_edge() {
        let a = Vector3::new(0., 0., 0.);
        let b = Vector3::new(1., 0., 0.);
        let c = Vector3::new(0., 1., 0.);
        let t0 = Triangle::new(a, b, c);

        let d = Vector3::new(0., 0., -1.);
        let e = Vector3::new(0., 0., 1.);
        let f = Vector3::new(-1., -1., 0.);
        let t1 = Triangle::new(d, e, f);

        let result = intersection_triangle_triangle(&t0, &t1).unwrap();

        assert_eq!(result[0], Vector3::new(0., 0., 0.));
        assert_eq!(result[1], Vector3::new(0., 0., 0.));
    }

    #[test]
    fn test_double_edge() {
        let a = Vector3::new(0., 0., 0.);
        let b = Vector3::new(1., 0., 0.);
        let c = Vector3::new(0., 1., 0.);
        let t0 = Triangle::new(a, b, c);

        let d = Vector3::new(0.1, 0.1, -1.);
        let e = Vector3::new(0.1, 0.1, 1.);
        let f = Vector3::new(-1., -1., 0.);
        let t1 = Triangle::new(d, e, f);

        let result = intersection_triangle_triangle(&t0, &t1).unwrap();

        assert_eq!(result[0], Vector3::new(0., 0., 0.));
        assert_eq!(result[1], Vector3::new(0.1, 0.1, 0.));
    }

    #[test]
    fn test_coplanar_identical() {
        let a = Vector3::new(0., 0., 1.);
        let b = Vector3::new(1., 0., 1.);
        let c = Vector3::new(0., 1., 1.);
        let t = Triangle::new(a, b, c);

        let result = intersection_triangle_triangle(&t, &t);

        assert!(result.is_none());
    }

    #[test]
    fn test_coplanar_offset() {
        let a = Vector3::new(0., 0., 1.);
        let b = Vector3::new(1., 0., 1.);
        let c = Vector3::new(0., 1., 1.);
        let t0 = Triangle::new(a, b, c);

        let d = Vector3::new(0., 0., 2.);
        let e = Vector3::new(1., 0., 2.);
        let f = Vector3::new(0., 1., 2.);
        let t1 = Triangle::new(d, e, f);

        let result = intersection_triangle_triangle(&t0, &t1);

        assert!(result.is_none());
    }

    #[test]
    fn test_miss() {
        let a = Vector3::new(0., 0., 1.);
        let b = Vector3::new(1., 0., 1.);
        let c = Vector3::new(0., 1., 1.);
        let t0 = Triangle::new(a, b, c);

        let d = Vector3::new(0., 1., 2.);
        let e = Vector3::new(1., 0., 2.);
        let f = Vector3::new(0., 1., 2.);
        let t1 = Triangle::new(d, e, f);

        let result = intersection_triangle_triangle(&t0, &t1);

        assert!(result.is_none());
    }
}
