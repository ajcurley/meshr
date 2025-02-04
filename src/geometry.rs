pub mod aabb;
pub mod line;
pub mod ray;
pub mod sphere;
pub mod triangle;
pub mod vector3;

// Private modules
mod collision;

// Re-exports
pub use aabb::Aabb;
pub use line::Line;
pub use ray::Ray;
pub use sphere::Sphere;
pub use triangle::Triangle;
pub use vector3::Vector3;

/// Geometric tolerance
pub const EPSILON: f64 = 1e-8;

/// Get the shortest distance between two geometric entities
pub trait Distance<T> {
    fn distance(&self, other: &T) -> f64;
}

/// Check for a spatial intersection between two geometric entities
pub trait Intersects<T> {
    fn intersects(&self, other: &T) -> bool;
}

/// Get the intersection result between two geometry entities
pub trait Intersection<T> {
    fn intersection(&self, other: &T) -> Option<Geometry>;
}

/// Clip to the geometry
pub trait Clip<T> {
    fn clip(&self, other: &T) -> Option<Geometry>;
}

#[derive(Debug, Clone)]
pub enum Geometry {
    Aabb(Aabb),
    Line(Line),
    Point(Vector3),
    Ray(Ray),
    Sphere(Sphere),
    Triangle(Triangle),
}

impl From<Aabb> for Geometry {
    fn from(value: Aabb) -> Geometry {
        Geometry::Aabb(value)
    }
}

impl From<Line> for Geometry {
    fn from(value: Line) -> Geometry {
        Geometry::Line(value)
    }
}

impl From<Vector3> for Geometry {
    fn from(value: Vector3) -> Geometry {
        Geometry::Point(value)
    }
}
