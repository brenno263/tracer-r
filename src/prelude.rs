pub use crate::{
    bounded_volume_hierarchy::{BVHBuildNode, Bounds, BVHFlat},
    camera::Camera,
    image_handling::{ImageBuffer, PixelF},
    material::Material,
    primitives::Primitive,
    raytracer::Raytracer,
    traits::*,
    utils::{lerp, parse_pair},
    vectors::V3,
};

// If you're not familiar with a prelude, it re-exports an essential set of the most commonly needed
// functionalities, so that they can all be imported in one line.
// This one is a little too eager, I'll admit. It should be cropped down a bit.
