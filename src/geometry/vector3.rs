use crate::geometry::collision;
use crate::geometry::{Aabb, Sphere};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    /// Construct a Vector3 from its components
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    /// Construct a Vector3 of all zeros
    pub fn zeros() -> Vector3 {
        Vector3::new(0., 0., 0.)
    }

    /// Construct a Vector3 of all ones
    pub fn ones() -> Vector3 {
        Vector3::new(1., 1., 1.)
    }

    /// Compute the dot product u * v
    pub fn dot(u: &Vector3, v: &Vector3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    /// Compute the cross product u x v
    pub fn cross(u: &Vector3, v: &Vector3) -> Vector3 {
        let i = u.y * v.z - u.z * v.y;
        let j = u.z * v.x - u.x * v.z;
        let k = u.x * v.y - u.y * v.x;
        Vector3::new(i, j, k)
    }

    /// Get the x-component
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Get the y-component
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Get the z-component
    pub fn z(&self) -> f64 {
        self.z
    }

    /// Compute the angle (in radians) between u and v
    pub fn angle(u: &Vector3, v: &Vector3) -> f64 {
        (Vector3::dot(&u, &v) / (u.mag() * v.mag()))
            .clamp(-1., 1.)
            .acos()
    }

    /// Get the magnitude
    pub fn mag(&self) -> f64 {
        Vector3::dot(self, self).sqrt()
    }

    /// Get the unit (magnitude = 1)
    pub fn unit(&self) -> Vector3 {
        *self / self.mag()
    }

    /// Get the inverse
    pub fn inv(&self) -> Vector3 {
        1. / *self
    }

    /// Get the absolute vector
    pub fn abs(&self) -> Vector3 {
        Vector3::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    /// Get the index of the minimal component
    pub fn min_index(&self) -> usize {
        let mut index = 0;
        let mut value = self.x;

        for i in 1..3 {
            if self[i] < value {
                index = i;
                value = self[i];
            }
        }

        index
    }

    /// Get the index of the maximal component
    pub fn max_index(&self) -> usize {
        let mut index = 0;
        let mut value = self.x;

        for i in 1..3 {
            if self[i] > value {
                index = i;
                value = self[i];
            }
        }

        index
    }
}

impl std::ops::Index<usize> for Vector3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of range"),
        }
    }
}

impl std::ops::IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of range"),
        }
    }
}

impl std::ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, v: Vector3) -> Self::Output {
        Vector3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl std::ops::Add<f64> for Vector3 {
    type Output = Vector3;

    fn add(self, v: f64) -> Self::Output {
        Vector3::new(self.x + v, self.y + v, self.z + v)
    }
}

impl std::ops::Add<Vector3> for f64 {
    type Output = Vector3;

    fn add(self, v: Vector3) -> Self::Output {
        Vector3::new(self + v.x, self + v.y, self + v.z)
    }
}

impl std::ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, v: Vector3) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

impl std::ops::AddAssign<f64> for Vector3 {
    fn add_assign(&mut self, v: f64) {
        self.x += v;
        self.y += v;
        self.z += v;
    }
}

impl std::ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, v: Vector3) -> Self::Output {
        Vector3::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl std::ops::Sub<f64> for Vector3 {
    type Output = Vector3;

    fn sub(self, v: f64) -> Self::Output {
        Vector3::new(self.x - v, self.y - v, self.z - v)
    }
}

impl std::ops::Sub<Vector3> for f64 {
    type Output = Vector3;

    fn sub(self, v: Vector3) -> Self::Output {
        Vector3::new(self - v.x, self - v.y, self - v.z)
    }
}

impl std::ops::SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, v: Vector3) {
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
    }
}

impl std::ops::SubAssign<f64> for Vector3 {
    fn sub_assign(&mut self, v: f64) {
        self.x -= v;
        self.y -= v;
        self.z -= v;
    }
}

impl std::ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, v: Vector3) -> Self::Output {
        Vector3::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}

impl std::ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, v: f64) -> Self::Output {
        Vector3::new(self.x * v, self.y * v, self.z * v)
    }
}

impl std::ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, v: Vector3) -> Self::Output {
        Vector3::new(self * v.x, self * v.y, self * v.z)
    }
}

impl std::ops::MulAssign<Vector3> for Vector3 {
    fn mul_assign(&mut self, v: Vector3) {
        self.x *= v.x;
        self.y *= v.y;
        self.z *= v.z;
    }
}

impl std::ops::MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, v: f64) {
        self.x *= v;
        self.y *= v;
        self.z *= v;
    }
}

impl std::ops::Div<Vector3> for Vector3 {
    type Output = Vector3;

    fn div(self, v: Vector3) -> Self::Output {
        Vector3::new(self.x / v.x, self.y / v.y, self.z / v.z)
    }
}

impl std::ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, v: f64) -> Self::Output {
        Vector3::new(self.x / v, self.y / v, self.z / v)
    }
}

impl std::ops::Div<Vector3> for f64 {
    type Output = Vector3;

    fn div(self, v: Vector3) -> Self::Output {
        Vector3::new(self / v.x, self / v.y, self / v.z)
    }
}

impl std::ops::DivAssign<Vector3> for Vector3 {
    fn div_assign(&mut self, v: Vector3) {
        self.x /= v.x;
        self.y /= v.y;
        self.z /= v.z;
    }
}

impl std::ops::DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, v: f64) {
        self.x /= v;
        self.y /= v;
        self.z /= v;
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

impl crate::geometry::Intersects<Aabb> for Vector3 {
    fn intersects(&self, other: &Aabb) -> bool {
        collision::intersects::intersects_aabb_vector3(other, self)
    }
}

impl crate::geometry::Intersects<Sphere> for Vector3 {
    fn intersects(&self, other: &Sphere) -> bool {
        collision::intersects::intersects_sphere_vector3(other, self)
    }
}
