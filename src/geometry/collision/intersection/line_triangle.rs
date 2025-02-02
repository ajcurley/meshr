use crate::geometry::collision::Intersection;
use crate::geometry::{Line, Triangle, Vector3, EPSILON};

/// Compute the intersection point between a line segment and a triangle.
pub fn intersection_line_triangle(line: &Line, triangle: &Triangle) -> Option<Vector3> {
    let (v0, v1, v2) = triangle.vertices();
    let e0 = v1 - v0;
    let e1 = v2 - v0;

    let d = line.direction();
    let h = Vector3::cross(&d, &e1);
    let a = Vector3::dot(&e0, &h);

    if a.abs() < EPSILON {
        return None;
    }

    let f = 1. / a;
    let s = line[0] - v0;
    let u = f * Vector3::dot(&s, &h);

    if u < 0. || u > 1. {
        return None;
    }

    let q = Vector3::cross(&s, &e0);
    let v = f * Vector3::dot(&d, &q);

    if v < 0. || u + v > 1. {
        return None;
    }

    let t = f * Vector3::dot(&e1, &q);

    if t <= EPSILON {
        return None;
    }

    Some(line[0] + t * d)
}

/// Implement the Line/Triangle intersection for a Triangle
impl Intersection<Line> for Triangle {
    type Output = Vector3;

    fn intersection(&self, line: &Line) -> Option<Self::Output> {
        intersection_line_triangle(line, self)
    }
}

/// Implement the Line/Triangle intersection for a Line
impl Intersection<Triangle> for Line {
    type Output = Vector3;

    fn intersection(&self, triangle: &Triangle) -> Option<Self::Output> {
        intersection_line_triangle(self, triangle)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hit() {
        let p = Vector3::new(0.5, 0.5, 0.);
        let q = Vector3::new(0.5, 0.5, 2.);
        let l = Line::new(p, q);

        let a = Vector3::new(0., 0., 1.);
        let b = Vector3::new(0., 1., 1.);
        let c = Vector3::new(1., 0., 1.);
        let t = Triangle::new(a, b, c);

        let result = intersection_line_triangle(&l, &t).unwrap();
        assert_eq!(result, Vector3::new(0.5, 0.5, 1.));
    }

    #[test]
    fn hit_not_culled() {
        let p = Vector3::new(0.5, 0.5, 0.);
        let q = Vector3::new(0.5, 0.5, 2.);
        let l = Line::new(p, q);

        let a = Vector3::new(0., 0., 1.);
        let b = Vector3::new(1., 0., 1.);
        let c = Vector3::new(0., 1., 1.);
        let t = Triangle::new(a, b, c);

        let result = intersection_line_triangle(&l, &t).unwrap();
        assert_eq!(result, Vector3::new(0.5, 0.5, 1.));
    }

    #[test]
    fn miss() {
        let p = Vector3::new(2., 2., 0.);
        let q = Vector3::new(2., 2., 1.);
        let l = Line::new(p, q);

        let a = Vector3::new(0., 0., 1.);
        let b = Vector3::new(0., 1., 1.);
        let c = Vector3::new(1., 0., 1.);
        let t = Triangle::new(a, b, c);

        let result = intersection_line_triangle(&l, &t);
        assert!(result.is_none());
    }
}
