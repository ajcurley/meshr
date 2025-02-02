use crate::geometry::collision::Intersection;
use crate::geometry::{Line, Triangle};

/// Compute the intersection line between two triangles. If the intersection
/// represents a point, the line will be comprised of equal points.
pub fn intersection_triangle_triangle(t0: &Triangle, t1: &Triangle) -> Option<Line> {
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

impl Intersection<Triangle> for Triangle {
    type Output = Line;

    fn intersection(&self, triangle: &Triangle) -> Option<Self::Output> {
        intersection_triangle_triangle(self, triangle)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::geometry::Vector3;

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
