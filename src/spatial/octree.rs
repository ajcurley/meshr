use std::collections::HashMap;

use crate::geometry::{Aabb, Intersects};

/// Maximum depth of the Octree
const MAX_DEPTH: usize = (std::mem::size_of::<usize>() * 8 - 1) / 3;

/// Maximum number of items indexed on a leaf node
const MAX_ITEMS_PER_NODE: usize = 100;

#[derive(Debug, Clone)]
pub struct Octree<T>
where
    T: Intersects<Aabb>,
{
    nodes: HashMap<usize, OctreeNode>,
    items: Vec<T>,
}

impl<T> Octree<T>
where
    T: Intersects<Aabb>,
{
    /// Construct an Octree from its bounds
    pub fn new(bounds: Aabb) -> Octree<T> {
        Octree {
            nodes: HashMap::from([(1, OctreeNode::new(1, bounds))]),
            items: vec![],
        }
    }

    /// Get a borrowed reference to a node
    pub fn node(&self, code: usize) -> &OctreeNode {
        &self.nodes[&code]
    }

    /// Get a slice of the items
    pub fn items(&self) -> &[T] {
        &self.items
    }

    /// Insert an item which may be indexed on one or more nodes
    /// but must overlap with the Octree bounds.
    pub fn insert(&mut self, item: T) -> usize {
        let index = self.items.len();
        let mut queue = vec![1];
        let mut codes = vec![];

        while let Some(code) = queue.pop() {
            if let Some(node) = self.nodes.get_mut(&code) {
                if item.intersects(&node.bounds) {
                    if node.is_leaf {
                        node.items.push(index);
                        codes.push(code);
                    } else {
                        let mut children = node.children();
                        queue.append(&mut children);
                    }
                }
            }
        }

        if codes.is_empty() {
            panic!("item not inserted");
        }

        self.items.push(item);

        for code in codes {
            if self.nodes[&code].should_split() {
                self.split(code);
            }
        }

        index
    }

    /// Split an internal (non-leaf) node and redistribute any indexed
    /// items amongst the children leaf nodes.
    pub fn split(&mut self, code: usize) {
        if let Some(node) = self.nodes.get_mut(&code) {
            if !node.can_split() {
                panic!("octree node cannot be split");
            }

            let children = node.children();
            let bounds = node.bounds;
            let items = node.items.clone();

            node.is_leaf = false;
            node.items.clear();

            for (octant, &child_code) in children.iter().enumerate() {
                let child_bounds = bounds.octant(octant);
                let mut child_node = OctreeNode::new(child_code, child_bounds);

                for &item in items.iter() {
                    if self.items[item].intersects(&child_bounds) {
                        child_node.items.push(item);
                    }
                }

                self.nodes.insert(child_code, child_node);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct OctreeNode {
    code: usize,
    bounds: Aabb,
    is_leaf: bool,
    items: Vec<usize>,
}

impl OctreeNode {
    /// Construct an OctreeNode from its code and bounds
    pub fn new(code: usize, bounds: Aabb) -> OctreeNode {
        OctreeNode {
            code,
            bounds,
            is_leaf: true,
            items: vec![],
        }
    }

    /// Get the code
    pub fn code(&self) -> usize {
        self.code
    }

    /// Get the bounds
    pub fn bounds(&self) -> Aabb {
        self.bounds
    }

    /// Check if the node is a leaf
    pub fn is_leaf(&self) -> bool {
        self.is_leaf
    }

    /// Get a slice of indexed items
    pub fn items(&self) -> &[usize] {
        &self.items
    }

    /// Get the depth of the node
    pub fn depth(&self) -> usize {
        (0..MAX_DEPTH + 1)
            .find(|d| self.code >> d * 3 == 1)
            .expect("invalid octree code")
    }

    /// Get the children location codes
    pub fn children(&self) -> Vec<usize> {
        (0..8).map(|o| (self.code << 3) | o).collect()
    }

    /// Check if the node can be split
    pub fn can_split(&self) -> bool {
        self.is_leaf && self.depth() < MAX_DEPTH
    }

    /// Check if the node should be split
    pub fn should_split(&self) -> bool {
        self.items.len() > MAX_ITEMS_PER_NODE && self.can_split()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::geometry::Vector3;

    #[test]
    fn insert_single() {
        let point = Vector3::zeros();
        let bounds = Aabb::unit();
        let mut octree = Octree::<Vector3>::new(bounds);
        let index = octree.insert(point);

        assert_eq!(0, index);
        assert_eq!(1, octree.nodes.len());
        assert_eq!(1, octree.items.len());

        let items = octree.node(1).items();
        assert_eq!(1, items.len());
        assert_eq!(0, items[0]);
    }

    #[test]
    fn insert_split() {
        let bounds = Aabb::unit();
        let mut octree = Octree::<Vector3>::new(bounds);
        let count = MAX_ITEMS_PER_NODE + 1;

        for i in 0..count {
            let v = 0.5 * (i as f64) / (count as f64 - 1.) - 0.25;
            let p = Vector3::new(v, v, v);
            octree.insert(p);
        }

        assert_eq!(9, octree.nodes.len());
        assert_eq!(count, octree.items.len());

        assert_eq!(0, octree.node(1).items.len());
        assert_eq!(count / 2 + 1, octree.node(8).items.len());
        assert_eq!(1, octree.node(9).items.len());
        assert_eq!(1, octree.node(10).items.len());
        assert_eq!(1, octree.node(11).items.len());
        assert_eq!(1, octree.node(12).items.len());
        assert_eq!(1, octree.node(14).items.len());
        assert_eq!(1, octree.node(14).items.len());
        assert_eq!(count / 2 + 1, octree.node(15).items.len());
    }

    #[test]
    #[should_panic]
    fn insert_no_overlap() {
        let point = Vector3::new(1., 1., 1.);
        let bounds = Aabb::unit();
        let mut octree = Octree::<Vector3>::new(bounds);
        octree.insert(point);
    }
}
