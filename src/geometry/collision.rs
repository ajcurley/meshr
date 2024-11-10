mod intersects;

pub trait Intersects<T> {
    fn intersects(&self, other: &T) -> bool;
}
