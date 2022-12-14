use std::fmt::Debug;

use crate::bounded_volume_hierarchy::Bounds;
use crate::camera::Camera;
use crate::image_handling::PixelF;
use crate::ray::Ray;
use crate::raytracer::Collision;

/// relatively generic way of using canvases, so that we can adapt to use a variety of output methods.
pub trait Canvas {
    fn put_pixel(&mut self, x: usize, y: usize, pixel: PixelF);
    fn bounds(&self) -> (usize, usize);
    fn offset(&self) -> (usize, usize);
}

/// This didn't really need to be a trait, but I do have dreams of implementing a rasterizer to be used
/// alongside the raytracer, which this would enable.
pub trait Renderer {
    fn render<C: Canvas>(
        &self,
        scene: &dyn Drawable,
        canvas: &mut C,
        camera: &Camera,
    ) -> Result<(), String>;
}

/// This trait describes anything that can be intersected with, and as such drawn by our raytracer.
/// Notable items that fit this are Primitives, collections of Primitives (there's a helper method here for exactly that),
/// and our BVH and LinearBVH.
pub trait Drawable {
    fn intersect(&self, ray: Ray) -> Option<Collision>;
}

/// Intersect a collection of Drawables. This should be a generic trait implementation, but I can't 
/// figure out how to do that at the moment.
pub fn intersect_collection<I: IntoIterator>(collection: I, mut ray: Ray) -> Option<Collision>
where
    I::Item: Drawable,
{
    let mut out = None;
    for ref el in collection {
        if let Some(coll) = el.intersect(ray) {
            ray.max = coll.t;
            out = Some(coll);
        }
    }
    out
}

/// Something that is boundable is both drawable, and can describe its bounds.
pub trait Boundable: Drawable {
    fn bounds(&self) -> Bounds;
}

/// This went unused, but was a generic weighted mean trait, allowing the operation to be done
/// on a variety of iterators.
pub trait WeightedMean<T = Self>: Sized {
    fn weighted_mean(it: impl Iterator<Item = (T, f64)>) -> Option<Self>;
}

impl Debug for dyn Drawable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Drawable").finish()
    }
}

impl Debug for dyn Boundable + Sync + Send {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Boundable").finish()
    }
}
