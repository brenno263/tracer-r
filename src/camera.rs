use crate::V3;
use crate::Ray;

pub struct Camera {
	position: V3,
	// position of the upper-left corner of the viewport relative to position
	upper_left: V3,
	// vector from left to right viewport border
	horizontal: V3,
	// vector from bottom to top viewport border
	vertical: V3,
}

impl Camera {
	pub fn new(position: V3, direction: V3, up: V3, fov: f32, aspect_r: f32) -> Camera {
		let z = direction.normalized();
		let y = up.normalized();
		let x = z.cross(&y);
		let vertical = y * (2.0 * f32::tan(fov / 2.0));
		let horizontal = x * (-2.0 * f32::tan(fov / 2.0) * aspect_r);
		let upper_left = position + z - (horizontal * 0.5) + (vertical * 0.5);
		Camera { position, upper_left, horizontal, vertical}
	}

	/// takes x, y in [0, 1)x[0, 1)
	pub fn get_ray(self: &Self, x: f32, y: f32) -> Ray {
		Ray {
			origin: self.position,
			dir: (self.upper_left + (self.horizontal * x) - (self.vertical * y) - self.position).normalized()
		}
	}
}
