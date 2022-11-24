use crate::vectors::*;

pub trait Drawable {
	fn intersect(self: &Self, ray: &Ray) -> Option<Collision>;
}

pub struct Scene<'a> {
	pub elements: Vec<&'a dyn Drawable>,
}

impl Drawable for Scene<'_> {
    fn intersect(self: &Self, ray: &Ray) -> Option<Collision> {
        for el in &self.elements {
			if let Some(coll) = el.intersect(ray) {
				return Some(coll);
			}
		}
		None
    }
}
