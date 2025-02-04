use crate::geometry::collision;
use crate::geometry::{Aabb, Geometry, Line, Ray, Vector3};

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    p: Vector3,
    q: Vector3,
    r: Vector3,
}

impl Triangle {
    /// Construct a Triangle from its vertices
    pub fn new(p: Vector3, q: Vector3, r: Vector3) -> Triangle {
        Triangle { p, q, r }
    }

    /// Get the vertices as a tuple
    pub fn vertices(&self) -> (Vector3, Vector3, Vector3) {
        (self.p, self.q, self.r)
    }

    /// Get the normal
    pub fn normal(&self) -> Vector3 {
        let u = self.q - self.p;
        let v = self.r - self.p;
        Vector3::cross(&u, &v)
    }

    /// Get the unit normal
    pub fn unit_normal(&self) -> Vector3 {
        self.normal().unit()
    }

    /// Get the area
    pub fn area(&self) -> f64 {
        self.normal().mag() * 0.5
    }

    /// Get the center
    pub fn center(&self) -> Vector3 {
        (self.p + self.q + self.r) / 3.
    }

    /// Get the barycenter
    pub fn barycenter(&self) -> Vector3 {
        let i = self.q - self.p;
        let j = self.r - self.q;
        let k = self.p - self.r;

        let dii = Vector3::dot(&i, &i);
        let dij = Vector3::dot(&i, &j);
        let djj = Vector3::dot(&j, &j);
        let dki = Vector3::dot(&k, &i);
        let dkj = Vector3::dot(&j, &j);

        let d = dii * djj - dij * dij;
        let v = (djj * dki - dij * dkj) / d;
        let w = (dii * dkj - dii * dki) / d;
        let u = 1. - v - w;

        Vector3::new(u, v, w)
    }

    /// Get the edges of the triangle
    pub fn edges(&self) -> [Line; 3] {
        [
            Line::new(self.q, self.p),
            Line::new(self.r, self.q),
            Line::new(self.p, self.r),
        ]
    }
}

impl std::ops::Index<usize> for Triangle {
    type Output = Vector3;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.p,
            1 => &self.q,
            2 => &self.r,
            _ => panic!("index out of range"),
        }
    }
}

impl std::ops::IndexMut<usize> for Triangle {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.p,
            1 => &mut self.q,
            2 => &mut self.r,
            _ => panic!("index out of range"),
        }
    }
}

impl crate::geometry::Intersects<Aabb> for Triangle {
    fn intersects(&self, other: &Aabb) -> bool {
        collision::intersects::intersects_aabb_triangle(other, self)
    }
}

impl crate::geometry::Intersects<Ray> for Triangle {
    fn intersects(&self, other: &Ray) -> bool {
        collision::intersects::intersects_ray_triangle(other, self)
    }
}

impl crate::geometry::Intersects<Triangle> for Triangle {
    fn intersects(&self, other: &Triangle) -> bool {
        collision::intersects::intersects_triangle_triangle(self, other)
    }
}

impl crate::geometry::Intersects<Vector3> for Triangle {
    fn intersects(&self, _other: &Vector3) -> bool {
        // TODO: implement
        unimplemented!();
    }
}

impl crate::geometry::Intersection<Line> for Triangle {
    fn intersection(&self, other: &Line) -> Option<Geometry> {
        collision::intersection::intersection_line_triangle(other, self)
    }
}

impl crate::geometry::Intersection<Triangle> for Triangle {
    fn intersection(&self, other: &Triangle) -> Option<Geometry> {
        collision::intersection::intersection_triangle_triangle(self, other)
    }
}
