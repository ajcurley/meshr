pub mod octree;

// Re-exports
pub use octree::Octree;

/// Find items spatial intersecting the query
pub trait Query<Q> {
    fn query(&self, query: &Q) -> Vec<usize>;
}

/// Find items spatially intersecting the queries
pub trait QueryMany<Q> {
    fn query_many(&self, queries: &[Q]) -> Vec<Vec<usize>>;
}
