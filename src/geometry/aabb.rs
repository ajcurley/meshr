use crate::geometry::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Aabb {
    center: Vector3,
    halfsize: Vector3,
}
