use std::collections::VecDeque;

use crate::{
    primitives::Primitive,
    ray::Ray,
    raytracer::Collision,
    traits::{intersect_collection, Boundable, Drawable},
    vectors::V3,
};

/// Bounds defines an axis-aligned area in 3d space, bounded between its min_point and max_point
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Bounds {
    pub min_point: V3,
    pub max_point: V3,
}

impl Bounds {
	/// Take the union of two bounds, producing the minimal bound that contains both input bounds.
    fn union(b1: Bounds, b2: Bounds) -> Self {
        Bounds {
            min_point: V3 {
                x: f32::min(b1.min_point.x, b2.min_point.x),
                y: f32::min(b1.min_point.y, b2.min_point.y),
                z: f32::min(b1.min_point.z, b2.min_point.z),
            },
            max_point: V3 {
                x: f32::max(b1.max_point.x, b2.max_point.x),
                y: f32::max(b1.max_point.y, b2.max_point.y),
                z: f32::max(b1.max_point.z, b2.max_point.z),
            },
        }
    }

	/// Expand these bounds into a minimal bounds containing the original bounds and the supplied point.
    fn union_v3(b: Bounds, v: V3) -> Self {
        Bounds {
            min_point: V3 {
                x: f32::min(b.min_point.x, v.x),
                y: f32::min(b.min_point.y, v.y),
                z: f32::min(b.min_point.z, v.z),
            },
            max_point: V3 {
                x: f32::max(b.max_point.x, v.x),
                y: f32::max(b.max_point.y, v.y),
                z: f32::max(b.max_point.z, v.z),
            },
        }
    }

	/// Calculate the centroid of a bounds.
    fn centroid(&self) -> V3 {
        (self.min_point + self.max_point) * 0.5
    }

	/// Find the axis along which this bounds is longest.
    fn maximum_length_axis(&self) -> SplitAxis {
        let x_dim = self.max_point.x - self.min_point.x;
        let y_dim = self.max_point.y - self.min_point.y;
        let z_dim = self.max_point.z - self.min_point.z;

        if x_dim > y_dim && x_dim > z_dim {
            SplitAxis::X
        } else if y_dim > z_dim {
            SplitAxis::Y
        } else {
            SplitAxis::Z
        }
    }

	/// Determine if this bounds has zero area.
    fn is_singularity(&self) -> bool {
        self.min_point == self.max_point
    }

	/// Get this bounds' dimension along a supplied axis.
    fn dimension(&self, axis: SplitAxis) -> f32 {
        axis.proj(self.max_point) - axis.proj(self.min_point)
    }

	/// Check if a ray intersects these bounds
    fn intersects(&self, ray: &Ray) -> bool {
        let d_inv = V3::new(1. / ray.dir.x, 1. / ray.dir.y, 1. / ray.dir.z);
        self.intersects_with_dir_inv(ray, d_inv)
    }

    /// This version of intersection takes a precomputed inverted direction.
    /// This division is expensive and can be done just once for each ray.
    fn intersects_with_dir_inv(&self, ray: &Ray, d_inv: V3) -> bool {
        // We are really looking for the furthest intersection with a near-plane
        //  and the nearest intersection with a far-plane.
        // If the ray passes through the volume, the near-plane intersection
        //  will be closer than the far-plane intersect.
        let mut overall_t_near = ray.min;
        let mut overall_t_far = ray.max;

        for axis in [SplitAxis::X, SplitAxis::Y, SplitAxis::Z] {
            let mut t_near = (axis.proj(self.min_point) - axis.proj(ray.origin)) * axis.proj(d_inv);
            let mut t_far = (axis.proj(self.max_point) - axis.proj(ray.origin)) * axis.proj(d_inv);
            // Swap if necessary so these are ordered correctly
            if t_near > t_far {
                let temp = t_near;
                t_near = t_far;
                t_far = temp;
            }

            // Update our overall values with our new info.
            // Note that if we do a divide by zero and get a NaN,
            // those are required to fail every conditional.
            // So our conditional blocks won't overwrite with garbo data.
            if t_near > overall_t_near {
                overall_t_near = t_near;
            }
            if t_far < overall_t_far {
                overall_t_far = t_far;
            }

            if overall_t_near > overall_t_far {
                return false;
            }
        }

        true
    }
}

/// Contains a Primitive and a bounds and centroid with which do partitioning.
/// This will be converted back to a simple Primitive when partitioning is done,
/// discarding the additional information.
#[derive(Debug)]
pub struct BVHPrimitiveInfo {
    primitive: Primitive,
    bounds: Bounds,
    centroid: V3,
}

impl BVHPrimitiveInfo {
    pub fn new(primitive: Primitive) -> Self {
        let bounds = primitive.bounds();
        let centroid = bounds.centroid();
        BVHPrimitiveInfo {
            primitive,
            bounds,
            centroid,
        }
    }
}

// While this was intended to be turned into a LinearBVH before rendering,
// we go ahead and implement Drawable for all our BVH components so that
// it can also be drawn while just a tree.
impl Drawable for BVHPrimitiveInfo {
    fn intersect(&self, ray: Ray) -> Option<Collision> {
        self.primitive.intersect(ray)
    }
}

impl Drawable for &BVHPrimitiveInfo {
    fn intersect(&self, ray: Ray) -> Option<Collision> {
        self.primitive.intersect(ray)
    }
}

/// A way of referring to axes
#[derive(Debug)]
enum SplitAxis {
    X,
    Y,
    Z,
}

impl SplitAxis {
	/// Project a vector along some axis
    fn proj(&self, v: V3) -> f32 {
        match self {
            SplitAxis::X => v.x,
            SplitAxis::Y => v.y,
            SplitAxis::Z => v.z,
        }
    }
}

/// A node in our BVH tree
#[derive(Debug)]
pub struct BVHBuildNode {
    bounds: Bounds,
    split_axis: SplitAxis,
    n_prims: usize,
    pub n_nodes: usize,
    data: BVHBuildNodeData,
}

/// A BVHBuildNode can either have relevant BVHPrimitiveInfo items to query
/// or children nodes.
#[derive(Debug)]
enum BVHBuildNodeData {
    PrimInfos(Vec<BVHPrimitiveInfo>),
    Children(Box<(BVHBuildNode, BVHBuildNode)>),
}

impl BVHBuildNode {
    pub fn new(mut primatives: Vec<Primitive>, prims_per_leaf: usize) -> Self {
        let prim_infos: Vec<BVHPrimitiveInfo> = primatives
            .drain(..)
            .map(|prim| BVHPrimitiveInfo::new(prim))
            .collect();

        Self::recursive_build_bvh(prim_infos, prims_per_leaf)
    }

    fn recursive_build_bvh(
        mut prim_infos: Vec<BVHPrimitiveInfo>,
        prims_per_leaf: usize,
    ) -> BVHBuildNode {
        let n_prims = prim_infos.len();
        if n_prims <= prims_per_leaf {
            //Just make a leaf node and return. We can't subdivide further.
            BVHBuildNode::new_leaf(prim_infos)
        } else {
            // Choose a splitting dimension
            let centroid_avg = prim_infos
                .iter()
                .map(|p| p.centroid)
                .fold(V3::zero(), |acc, c| acc + c)
                / prim_infos.len() as f32;
            let starting_bounds = Bounds {
                min_point: centroid_avg,
                max_point: centroid_avg,
            };
            let centroid_bounds = prim_infos
                .iter()
                .map(|p| p.centroid)
                .fold(starting_bounds, Bounds::union_v3);
            let split_dim = centroid_bounds.maximum_length_axis();

            // If our area is a single point we can't do much here.
            if centroid_bounds.is_singularity() {
                return BVHBuildNode::new_leaf(prim_infos);
            }

            // Partition our infos into two sets
            let mid = partition::partition_index(prim_infos.as_mut_slice(), |p| {
                split_dim.proj(p.centroid) < split_dim.proj(centroid_avg)
            });

            let prim_infos_right = prim_infos.drain(mid..).collect();
            let prim_infos_left = prim_infos;

            // Call this method on those two sets to build children
            BVHBuildNode::new_interior(
                split_dim,
                Self::recursive_build_bvh(prim_infos_left, prims_per_leaf),
                Self::recursive_build_bvh(prim_infos_right, prims_per_leaf),
            )
        }
    }

    fn new_leaf(prim_infos: Vec<BVHPrimitiveInfo>) -> BVHBuildNode {
        let bounds = prim_infos
            .iter()
            .map(|pi| pi.bounds)
            .reduce(Bounds::union)
            .unwrap();
        let split_axis = bounds.maximum_length_axis();

        BVHBuildNode {
            bounds,
            split_axis,
            n_prims: prim_infos.len(),
            n_nodes: 1,
            data: BVHBuildNodeData::PrimInfos(prim_infos),
        }
    }

    /// Create a new interior node having two children.
	/// Note that the two children NEED to be contiguous.
    fn new_interior(split_axis: SplitAxis, c1: BVHBuildNode, c2: BVHBuildNode) -> BVHBuildNode {
        let bounds = Bounds::union(c1.bounds, c2.bounds);

        BVHBuildNode {
            bounds,
            split_axis,
            n_prims: c1.n_prims + c2.n_prims,
            n_nodes: c1.n_nodes + c2.n_nodes,
            data: BVHBuildNodeData::Children(Box::new((c1, c2))),
        }
    }

    fn is_leaf(&self) -> bool {
        matches!(self.data, BVHBuildNodeData::PrimInfos(_))
    }
}

impl Drawable for BVHBuildNode {
    fn intersect(&self, ray: Ray) -> Option<Collision> {
        if self.bounds.intersects(&ray) {
            match self.data {
                BVHBuildNodeData::PrimInfos(ref prim_infos) => {
                    intersect_collection(prim_infos, ray)
                }
                BVHBuildNodeData::Children(ref children) => {
                    if self.split_axis.proj(ray.dir) < 0. {
                        intersect_collection([&children.1, &children.0], ray)
                    } else {
                        intersect_collection([&children.0, &children.1], ray)
                    }
                }
            }
        } else {
            None
        }
    }
}

impl Drawable for &BVHBuildNode {
    fn intersect(&self, ray: Ray) -> Option<Collision> {
        (*self).intersect(ray)
    }
}

/// The FlatBVH is a flattened BVH tree, eschewing pointers for a contiguous chunk of memory.
/// It also crops extra information out of its primitives, terminating in Primitives rather than
/// BVHPrimitiveInfos.
pub struct BVHFlat {
    nodes: Vec<BVHFlatNode>,
}

impl BVHFlat {}

impl From<BVHBuildNode> for BVHFlat {
    fn from(root: BVHBuildNode) -> Self {
        // Since this is a flattened binary tree, we need our number of nodes to be a
        // power of two for child-getting logic to work out. Here we find the smallest
        // power of two which can contain our data.
        let mut n_nodes = 1;
        while n_nodes < root.n_nodes {
            n_nodes <<= 1;
        }
        let mut array: Vec<BVHFlatNode> = Vec::with_capacity(n_nodes);

        let mut current_node = root;
        let mut node_queue: VecDeque<BVHBuildNode> = VecDeque::with_capacity(128);

        loop {
            match current_node.data {
                BVHBuildNodeData::PrimInfos(mut prim_infos) => {
                    let prims: Vec<Primitive> =
                        prim_infos.drain(..).map(|pi| pi.primitive).collect();
                    array.push(BVHFlatNode {
                        split_axis: current_node.split_axis,
                        bounds: current_node.bounds,
                        data: BVHFlatNodeData::Prims(prims),
                    });
                }
                BVHBuildNodeData::Children(children) => {
                    let first_child_offset = array.len() + 1 + node_queue.len();
                    array.push(BVHFlatNode {
                        split_axis: current_node.split_axis,
                        bounds: current_node.bounds,
                        data: BVHFlatNodeData::Children((
                            first_child_offset,
                            first_child_offset + 1,
                        )),
                    });

                    node_queue.push_back(children.0);
                    node_queue.push_back(children.1);
                }
            };

            match node_queue.pop_front() {
                Some(popped) => {
                    current_node = popped;
                }
                None => {
                    break;
                }
            }
        }

        BVHFlat { nodes: array }
    }
}

struct BVHFlatNode {
    split_axis: SplitAxis,
    bounds: Bounds,
    data: BVHFlatNodeData,
}

enum BVHFlatNodeData {
    Children((usize, usize)),
    Prims(Vec<Primitive>),
}

impl Drawable for BVHFlat {
    fn intersect(&self, mut ray: Ray) -> Option<Collision> {
        let mut current_offset = 0;
        let mut offset_stack: Vec<usize> = Vec::with_capacity(128);
        let mut collision: Option<Collision> = None;

        let dir_inv = V3::new(1. / ray.dir.x, 1. / ray.dir.y, 1. / ray.dir.z);

        loop {
            let node = &self.nodes[current_offset];
            if node.bounds.intersects_with_dir_inv(&ray, dir_inv) {
                match node.data {
                    BVHFlatNodeData::Prims(ref prims) => {
                        for p in prims {
                            if let Some(coll) = p.intersect(ray) {
                                ray.max = coll.t;
                                collision = Some(coll);
                            }
                        }
                    }
                    BVHFlatNodeData::Children(child_offsets) => {
                        // If the direction is negative compared to this axis, visit
                        // the second (more positive) child first, since it's spacially
                        // closer.
                        if node.split_axis.proj(ray.dir) < 0. {
                            offset_stack.push(child_offsets.1);
                            offset_stack.push(child_offsets.0);
                        } else {
                            offset_stack.push(child_offsets.0);
                            offset_stack.push(child_offsets.1);
                        }
                    }
                }
            }

            match offset_stack.pop() {
                Some(popped) => {
                    current_offset = popped;
                }
                None => {
                    break;
                }
            }
        }

        collision
    }
}
