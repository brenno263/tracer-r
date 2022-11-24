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
			match el.intersect(ray) {
				Some(coll) => {return Some(coll)},
				None => {}
			}
		}

		None
    }
}
