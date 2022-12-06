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

pub trait Renderer {
    fn render<C: Canvas>(
        &self,
        scene: &dyn Drawable,
        canvas: &mut C,
        camera: &Camera,
    ) -> Result<(), String>;
}

pub trait Drawable {
    fn intersect(&self, ray: Ray) -> Option<Collision>;
}

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

// pub trait Partitionable: Drawable {
//     fn position(&self) -> V3;
//     fn intersects_plane(&self, plane: Plane) -> bool;
// }

pub trait Boundable: Drawable {
    fn bounds(&self) -> Bounds;
}

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
