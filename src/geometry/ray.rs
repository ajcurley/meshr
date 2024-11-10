use crate::geometry::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}
