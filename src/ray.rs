use crate::vectors::V3;

pub static RAY_MIN: f32 = 0.00001;
pub static RAY_MAX: f32 = 100_000.;

/// A Ray describes a ray of light cast out. It has an origin and a direction.
/// It also encodes a min and max, which are altered throughout rendering to
/// restrict calculations to a distance range.
#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: V3,
    pub dir: V3,
    pub min: f32,
    pub max: f32,
}

impl Ray {
    pub fn new(origin: V3, dir: V3) -> Self {
        Self {
            origin,
            dir,
            min: RAY_MIN,
            max: RAY_MAX,
        }
    }

    pub fn from_to(from: V3, to: V3) -> Self {
        Self {
            origin: from,
            dir: to - from,
            min: RAY_MIN,
            max: RAY_MAX,
        }
    }

    pub fn destination(&self, t: f32) -> V3 {
        self.origin + (self.dir * t)
    }

    pub fn get_faced_normal(&self, outward_normal: V3) -> V3 {
        //if the normal faces back towards us, this is the front face.
        let front_facing = self.dir.dot(&outward_normal) < 0f32;
        if front_facing {
            outward_normal
        } else {
            outward_normal * -1.
        }
    }
}
