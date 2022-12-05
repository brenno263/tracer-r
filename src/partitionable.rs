use std::{rc::Rc, sync::Arc};

use crate::{
    raytracer::Collision,
    traits::{Drawable, Partitionable},
    vectors::{self, Plane, Ray, V3},
};

pub struct PScene {
    elements: Vec<Arc<dyn Partitionable + Send + Sync>>,
}

impl PScene {
    pub fn new(elements: Vec<Arc<dyn Partitionable + Send + Sync>>) -> Self {
        Self { elements }
    }

    /// Return whether this scene has enough elements to be worth partitioning.
    pub fn worth_partitioning(&self) -> bool {
        self.elements.len() > 1
    }
}

impl Drawable for PScene {
    fn intersect(&self, ray: Ray, min: f32, max: f32) -> Option<Collision> {
        let mut closest_so_far = max;
        let mut out = None;

        for el in &self.elements {
            if let Some(coll) = el.intersect(ray, min, closest_so_far) {
                out = Some(coll);
                closest_so_far = coll.t;
            }
        }
        out
    }
}

struct PartitionNode {
    plane: Plane,
    data: PartitionData,
}

enum PartitionData {
    Part {
        left: Box<PartitionNode>,
        right: Box<PartitionNode>,
    },
    Scene {
        left: PScene,
        right: PScene,
    },
}

// impl Drawable!!

impl PartitionNode {
    ///Attempts to partition the scene multiple times down to the specified depth. Stops early if a scene has less than two elements.
    pub fn multi_partition_scene(scene: PScene, max_depth: u32) -> PartitionNode {
        let first_part = Self::partition_scene(scene);
        Self::multi_partition_recursive(first_part, max_depth, 1)
    }

    fn multi_partition_recursive(
        partition: PartitionNode,
        max_depth: u32,
        depth: u32,
    ) -> PartitionNode {
        if depth > max_depth {
            return partition;
        }

        if let PartitionData::Scene {
            ref left,
            ref right,
        } = partition.data
        {
            if !left.worth_partitioning() || !right.worth_partitioning() {
                return partition;
            }
        }

        let (part_left, part_right) = match partition.data {
            PartitionData::Part { left, right } => (
                Self::multi_partition_recursive(*left, max_depth, depth + 1),
                Self::multi_partition_recursive(*right, max_depth, depth + 1),
            ),
            PartitionData::Scene {
                left: left_scene,
                right: right_scene,
            } => (
                Self::partition_scene(left_scene),
                Self::partition_scene(right_scene),
            ),
        };

        PartitionNode {
            plane: partition.plane,
            data: PartitionData::Part {
                left: Box::new(part_left),
                right: Box::new(part_right),
            },
        }
    }

    fn partition_scene(scene: PScene) -> PartitionNode {
        //Find the center of mass for the center point of our plane.
        //Bisect this with a plane, which is currently random but SHOULD use the linear regression as its normal.
        let average: V3 = scene.elements.iter().map(|el| el.position()).sum::<V3>() * 1.
            / scene.elements.len() as f32;
        let normal = V3::random_on_unit_sphere();
        let bisection_plane = Plane {
            point: average,
            normal,
        };

        let mut left_elements = Vec::with_capacity(scene.elements.len());
        let mut right_elements = Vec::with_capacity(scene.elements.len());

        for element in scene.elements {
            if element.intersects_plane(bisection_plane) {
                left_elements.push(element.clone());
                right_elements.push(element);
            }
            //TODO determine sidedness
        }

        left_elements.shrink_to_fit();
        right_elements.shrink_to_fit();

        PartitionNode {
            plane: bisection_plane,
            data: PartitionData::Scene {
                left: PScene {
                    elements: left_elements,
                },
                right: PScene {
                    elements: right_elements,
                },
            },
        }
    }
}
