use crate::vectors::*;
use crate::Drawable;

pub struct Sphere<'a> {
	center: V3,
	radius: f32,
	material: &'a dyn Material,
}

impl Sphere<'_> {
	pub fn new(center: V3, radius: f32, material: &dyn Material) -> Sphere {
		Sphere {center, radius, material}
	}
}

impl Drawable for Sphere<'_> {
	fn intersect(self: &Self, ray: &Ray) -> Option<Collision> {
		//t^2(D*D) + 2t(D*(O-C)) + (O-C) * (O-C) - r^2 = 0

		let center_to_ray_origin: V3 = ray.origin - self.center;
		let a = ray.dir.dot(&ray.dir);
		let half_b = ray.dir.dot(&center_to_ray_origin);
		let c = center_to_ray_origin.dot(&center_to_ray_origin) - (self.radius * self.radius);

		let discriminant = half_b * half_b - a * c;
		if discriminant < 0f32 {
			return Option::None;
		}

		let sqrtd = discriminant.sqrt();

		//get the closer root
		let root = (-half_b - sqrtd) / a;

		if root <= 0f32 {
			return Option::None;
		}

		let point = ray.origin + (ray.dir * root);
		let raw_normal = point - self.center;

		Option::Some(Collision::new(ray.clone(), point, raw_normal, root, self.material))
	}
}
