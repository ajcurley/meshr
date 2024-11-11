use crate::geometry::collision::Intersects;
use crate::geometry::{Aabb, Triangle, Vector3};

/// Check for an AABB/Triangle spatial intersection
/// - source: https://fileadmin.cs.lth.se/cs/Personal/Tomas_Akenine-Moller/code/tribox3.txt
fn intersects_aabb_triangle(a: &Aabb, t: &Triangle) -> bool {
    let center = a.center();
    let halfsize = a.halfsize();

    // Shift the system such that the AABB is at the origin
    let v0 = t[0] - center;
    let v1 = t[1] - center;
    let v2 = t[2] - center;

    // Calculate the edge vectors
    let e0 = v1 - v0;
    let e1 = v2 - v1;
    let e2 = v0 - v2;

    // Bullet #3 - 9 axis tests
    let fex = e0[0].abs();
    let fey = e0[1].abs();
    let fez = e0[2].abs();

    if !axistest_x01(e0[2], e0[2], fez, fey, v0, v2, halfsize) {
        return false;
    }

    if !axistest_y02(e0[2], e0[0], fez, fex, v0, v2, halfsize) {
        return false;
    }

    if !axistest_z12(e0[1], e0[0], fey, fex, v1, v2, halfsize) {
        return false;
    }

    let fex = e1[0].abs();
    let fey = e1[1].abs();
    let fez = e1[2].abs();

    if !axistest_x01(e1[2], e1[1], fez, fey, v0, v2, halfsize) {
        return false;
    }

    if !axistest_y02(e1[2], e1[0], fez, fex, v0, v2, halfsize) {
        return false;
    }

    if !axistest_z0(e1[1], e1[0], fey, fex, v0, v1, halfsize) {
        return false;
    }

    let fex = e2[0].abs();
    let fey = e2[1].abs();
    let fez = e2[2].abs();

    if !axistest_x2(e2[2], e2[1], fez, fey, v0, v1, halfsize) {
        return false;
    }

    if !axistest_y1(e2[2], e2[0], fez, fex, v0, v1, halfsize) {
        return false;
    }

    if !axistest_z12(e2[1], e2[0], fey, fex, v1, v2, halfsize) {
        return false;
    }

    // Bullet #1 - check for an intersection between the AABB and the
    // bounds of the triangle.
    for i in 0..3 {
        let min = v0[i].min(v1[i]).min(v2[i]);
        let max = v0[i].max(v1[i]).max(v2[i]);

        if min > halfsize[i] || max < -halfsize[i] {
            return false;
        }
    }

    // Bullet #2 - check for an instersection between the AABB and the
    // plane of the triangle.
    let normal = Vector3::cross(&e0, &e1);

    if !plane_box_overlap(normal, v0, halfsize) {
        return false;
    }

    true
}

/// Check for a spatial intersection for a unit AABB and the plane of the triangle
fn plane_box_overlap(normal: Vector3, v: Vector3, halfsize: Vector3) -> bool {
    let mut min = Vector3::zeros();
    let mut max = Vector3::zeros();

    for i in 0..3 {
        if normal[i] > 0. {
            min[i] = -halfsize[i] - v[i];
            max[i] = halfsize[i] - v[i];
        } else {
            min[i] = halfsize[i] - v[i];
            max[i] = -halfsize[i] - v[i];
        }
    }

    Vector3::dot(&normal, &min) <= 0. && Vector3::dot(&normal, &max) >= 0.
}

/// Axis test X01
fn axistest_x01(a: f64, b: f64, fa: f64, fb: f64, v0: Vector3, v2: Vector3, h: Vector3) -> bool {
    let p0 = a * v0[1] - b * v0[2];
    let p2 = a * v2[1] - b * v2[2];
    let (min, max) = if p0 < p2 { (p0, p2) } else { (p2, p0) };
    let rad = fa * h[1] + fb * h[2];
    !(min > rad || max < -rad)
}

/// Axis test X2
fn axistest_x2(a: f64, b: f64, fa: f64, fb: f64, v0: Vector3, v1: Vector3, h: Vector3) -> bool {
    let p0 = a * v0[1] - b * v0[2];
    let p1 = a * v1[1] - b * v1[2];
    let (min, max) = if p0 < p1 { (p0, p1) } else { (p1, p0) };
    let rad = fa * h[1] + fb * h[2];
    !(min > rad || max < -rad)
}

/// Axis test Y02
fn axistest_y02(a: f64, b: f64, fa: f64, fb: f64, v0: Vector3, v2: Vector3, h: Vector3) -> bool {
    let p0 = -a * v0[0] + b * v0[2];
    let p2 = -a * v2[0] + b * v2[2];
    let (min, max) = if p0 < p2 { (p0, p2) } else { (p2, p0) };
    let rad = fa * h[0] + fb * h[2];
    !(min > rad || max < -rad)
}

/// Axis test Y1
fn axistest_y1(a: f64, b: f64, fa: f64, fb: f64, v0: Vector3, v1: Vector3, h: Vector3) -> bool {
    let p0 = -a * v0[0] + b * v0[2];
    let p1 = -a * v1[0] + b * v1[2];
    let (min, max) = if p0 < p1 { (p0, p1) } else { (p1, p0) };
    let rad = fa * h[0] + fb * h[2];
    !(min > rad || max < -rad)
}

/// Axis test Z12
fn axistest_z12(a: f64, b: f64, fa: f64, fb: f64, v1: Vector3, v2: Vector3, h: Vector3) -> bool {
    let p1 = a * v1[0] - b * v1[1];
    let p2 = a * v2[0] - b * v2[1];
    let (min, max) = if p1 < p2 { (p1, p2) } else { (p2, p1) };
    let rad = fa * h[0] + fb * h[1];
    !(min > rad || max < -rad)
}

/// Axis test Z0
fn axistest_z0(a: f64, b: f64, fa: f64, fb: f64, v0: Vector3, v1: Vector3, h: Vector3) -> bool {
    let p0 = a * v0[0] - b * v0[1];
    let p1 = a * v1[0] - b * v1[1];
    let (min, max) = if p0 < p1 { (p0, p1) } else { (p1, p0) };
    let rad = fa * h[0] + fb * h[1];
    !(min > rad || max < -rad)
}

impl Intersects<Aabb> for Triangle {
    fn intersects(&self, a: &Aabb) -> bool {
        intersects_aabb_triangle(a, self)
    }
}

impl Intersects<Triangle> for Aabb {
    fn intersects(&self, t: &Triangle) -> bool {
        intersects_aabb_triangle(self, t)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hit_overlap_inside() {
        let a = Aabb::unit();
        let p = Vector3::new(0.1, 0.1, 0.1);
        let q = Vector3::new(0.1, 0.1, 0.3);
        let r = Vector3::new(0.1, 0.3, 0.1);
        let t = Triangle::new(p, q, r);

        assert!(intersects_aabb_triangle(&a, &t));
    }

    #[test]
    fn hit_overlap_face() {
        let a = Aabb::unit();
        let p = Vector3::new(0.5, 0.5, 0.5);
        let q = Vector3::new(1.25, 0.75, 0.5);
        let r = Vector3::new(1.25, 0.25, 0.5);
        let t = Triangle::new(p, q, r);

        assert!(intersects_aabb_triangle(&a, &t));
    }

    #[test]
    fn hit_overlap_edge() {
        let a = Aabb::unit();
        let p = Vector3::new(0.25, -0.25, 0.5);
        let q = Vector3::new(1.25, 0.75, 0.5);
        let r = Vector3::new(1.25, -0.25, 0.5);
        let t = Triangle::new(p, q, r);

        assert!(intersects_aabb_triangle(&a, &t));
    }

    #[test]
    fn hit_overlap_full() {
        let a = Aabb::unit();
        let p = Vector3::new(-2., -1., 0.5);
        let q = Vector3::new(1.5, 3., 0.5);
        let r = Vector3::new(1.5, -1., 0.5);
        let t = Triangle::new(p, q, r);

        assert!(intersects_aabb_triangle(&a, &t));
    }

    #[test]
    fn miss_triangle_bounds() {
        let a = Aabb::unit();
        let p = Vector3::new(0., 0., 2.);
        let q = Vector3::new(1., 0., 2.);
        let r = Vector3::new(1., 1., 2.);
        let t = Triangle::new(p, q, r);

        assert!(!intersects_aabb_triangle(&a, &t));
    }

    #[test]
    fn miss_triangle_plane() {
        let a = Aabb::unit();
        let p = Vector3::new(0.1, 1.1, 0.9);
        let q = Vector3::new(0.5, 0.8, 1.5);
        let r = Vector3::new(0.9, 1.1, 0.9);
        let t = Triangle::new(p, q, r);

        assert!(!intersects_aabb_triangle(&a, &t));
    }

    #[test]
    fn miss_axis_test_e0_x01() {
        let a = Aabb::unit();
        let p = Vector3::new(0.5, 1.1, 0.9);
        let q = Vector3::new(0.5, 0.8, 1.5);
        let r = Vector3::new(0.5, 1.3, 1.2);
        let t = Triangle::new(p, q, r);

        assert!(!intersects_aabb_triangle(&a, &t));
    }

    #[test]
    fn miss_axis_test_e0_y02() {
        let a = Aabb::unit();
        let p = Vector3::new(1.1, 0.5, 0.9);
        let q = Vector3::new(0.8, 0.5, 1.5);
        let r = Vector3::new(1.3, 0.5, 1.2);
        let t = Triangle::new(p, q, r);

        assert!(!intersects_aabb_triangle(&a, &t));
    }

    #[test]
    fn miss_axis_test_e0_z12() {
        let a = Aabb::unit();
        let p = Vector3::new(1.1, 0.9, 0.5);
        let q = Vector3::new(0.8, 1.5, 0.5);
        let r = Vector3::new(1.3, 1.2, 0.5);
        let t = Triangle::new(p, q, r);

        assert!(!intersects_aabb_triangle(&a, &t));
    }

    #[test]
    fn miss_axis_test_e1_x01() {
        let a = Aabb::unit();
        let p = Vector3::new(0.5, 1.3, 1.2);
        let q = Vector3::new(0.5, 1.1, 0.9);
        let r = Vector3::new(0.5, 0.8, 1.5);
        let t = Triangle::new(p, q, r);

        assert!(!intersects_aabb_triangle(&a, &t));
    }

    #[test]
    fn miss_axis_test_e1_y02() {
        let a = Aabb::unit();
        let p = Vector3::new(1.3, 0.5, 1.2);
        let q = Vector3::new(1.1, 0.5, 0.9);
        let r = Vector3::new(0.8, 0.5, 1.5);
        let t = Triangle::new(p, q, r);

        assert!(!intersects_aabb_triangle(&a, &t));
    }

    #[test]
    fn miss_axis_test_e1_z0() {
        let a = Aabb::unit();
        let p = Vector3::new(1.3, 1.2, 0.5);
        let q = Vector3::new(1.1, 0.9, 0.5);
        let r = Vector3::new(0.8, 1.5, 0.5);
        let t = Triangle::new(p, q, r);

        assert!(!intersects_aabb_triangle(&a, &t));
    }

    #[test]
    fn miss_axis_test_e2_x2() {
        let a = Aabb::unit();
        let p = Vector3::new(0.5, 0.8, 1.5);
        let q = Vector3::new(0.5, 1.3, 1.2);
        let r = Vector3::new(0.5, 1.1, 0.9);
        let t = Triangle::new(p, q, r);

        assert!(!intersects_aabb_triangle(&a, &t));
    }

    #[test]
    fn miss_axis_test_e2_y1() {
        let a = Aabb::unit();
        let p = Vector3::new(0.8, 0.5, 1.5);
        let q = Vector3::new(1.3, 0.5, 1.2);
        let r = Vector3::new(1.1, 0.5, 0.9);
        let t = Triangle::new(p, q, r);

        assert!(!intersects_aabb_triangle(&a, &t));
    }

    #[test]
    fn miss_axis_test_e2_z12() {
        let a = Aabb::unit();
        let p = Vector3::new(0.8, 1.5, 0.5);
        let q = Vector3::new(1.3, 1.2, 0.5);
        let r = Vector3::new(1.1, 0.9, 0.5);
        let t = Triangle::new(p, q, r);

        assert!(!intersects_aabb_triangle(&a, &t));
    }
}
