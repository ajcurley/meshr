use crate::geometry::{Aabb, Intersects};

pub fn check() -> bool {
    let a = Aabb::unit();
    let b = Aabb::unit();
    a.intersects(&b)
}
