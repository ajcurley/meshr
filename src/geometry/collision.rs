mod intersection;
mod intersects;

/// Geometric tolerance
const EPSILON: f64 = 1e-8;

/// Check for a spatial intersection between two geometric entities
pub trait Intersects<T> {
    fn intersects(&self, other: &T) -> bool;
}

/// Compute the spatial intersection between two geometric entities
pub trait Intersection<T> {
    type Output;

    fn intersection(&self, other: &T) -> Option<Self::Output>;
}
