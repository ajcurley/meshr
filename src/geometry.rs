pub mod aabb;
pub mod collision;
pub mod line;
pub mod ray;
pub mod sphere;
pub mod triangle;
pub mod vector3;

// Re-exports
pub use aabb::Aabb;
pub use collision::Intersects;
pub use line::Line;
pub use ray::Ray;
pub use sphere::Sphere;
pub use triangle::Triangle;
pub use vector3::Vector3;

/// Geometric tolerance
pub const EPSILON: f64 = 1e-8;
