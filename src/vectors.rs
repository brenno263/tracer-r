use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use rand::prelude::*;

use crate::image_handling::Pixel;

const EPLISON: f32 = 0.0001;

#[derive(Clone, Copy)]
pub struct V3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl V3 {
	pub fn new(x: f32, y: f32, z: f32) -> V3 {
		V3 {x, y, z}
	}

	pub fn zero() -> V3 {
		V3 {x:0.0, y:0.0, z:0.0}
	}

	pub fn one() -> V3 {
		V3 {x:1.0, y:1.0, z:1.0}
	}

	pub fn x() -> V3 {V3 { x: 1.0, y: 0.0, z: 0.0 }}
	pub fn y() -> V3 {V3 { x: 0.0, y: 1.0, z: 0.0 }}
	pub fn z() -> V3 {V3 { x: 0.0, y: 0.0, z: 1.0 }}

	pub fn dot(self: &Self, other: &Self) -> f32 {
		self.x * other.x + self.y * other.y + self.z * other.z
	}

	pub fn cross(self: &Self, other: &Self) -> V3 {
		V3 {
			x: self.y * other.z - self.z - other.y,
			y: self.z * other.x - self.x * other.z,
			z: self.x * other.y - self.y * other.x,
		}
	}

	pub fn magnitude(self: &Self) -> f32 {
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}

	pub fn normalized(self: &Self) -> V3 {
		let inverse_length = 1.0 / self.magnitude();
		*self * inverse_length
	}

	pub fn near_zero(self: &Self) -> bool {
		self.x.abs() < EPLISON && self.y.abs() < EPLISON && self.z.abs() < EPLISON
	}

	pub fn random() -> V3 {
		let mut rand = rand::thread_rng();
		V3 {x: rand.gen(), y: rand.gen(), z: rand.gen()}
	}

	pub fn random_in_range(min: f32, max: f32) -> V3 {
		assert!(min >= 0.0 && max >= 0.0);
		let delta = max - min;
		let mut rand = rand::thread_rng();
		V3 {
			x: rand.gen::<f32>() * delta + min,
			y: rand.gen::<f32>() * delta + min,
			z: rand.gen::<f32>() * delta + min,
		}
	}

	pub fn random_in_unit_sphere() -> V3 {
		let mut attempt = V3::random();
		while attempt.dot(&attempt) > 1.0 {
			attempt = V3::random();
		};
		attempt
	}

	pub fn random_on_unit_sphere() -> V3 {
		V3::random().normalized()
	}

}

impl Add for V3 {
	type Output = V3;

	fn add(self, rhs: V3) -> Self::Output {
		V3 {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
	}
}

impl Sub for V3 {
	type Output = V3;
	
	fn sub(self, rhs: Self) -> Self::Output {
		V3 {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
	}
}

impl Mul<f32> for V3 {
	type Output = V3;

	fn mul(self, rhs: f32) -> Self::Output {
		V3 {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
	}
}

impl Div<f32> for V3 {
	type Output = V3;

	fn div(self, rhs: f32) -> Self::Output {
		let s = 1.0 / rhs;
		V3 {x: self.x * s, y: self.y * s, z: self.z * s}
	}
}

impl std::fmt::Display for V3 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({},{},{})", self.x, self.y, self.z)
	}
}

#[derive(Clone, Copy)]
pub struct Ray {
	pub origin: V3,
	pub dir: V3,
}

impl Ray {
	pub fn from_to(from: V3, to: V3) -> Ray {
		Ray {origin: from.clone(), dir: to - from}
	}

	pub fn destination(self: &Self, t: f32) -> V3 {
		self.origin + (self.dir * t)
	}

	pub fn get_faced_normal(self: &Self, outward_normal: V3) -> V3 {
		//if the normal faces back towards us, this is the front face.
		let front_facing = self.dir.dot(&outward_normal) < 0f32;
		if front_facing {outward_normal.clone()} else {outward_normal.clone() * -1f32}
	}
}

#[derive(Clone, Copy)]
pub struct Collision {
	pub ray_in: Ray,
	pub ray_out: Ray,
	pub point: V3,
	pub normal: V3,
	pub t: f32,
	pub front_facing: bool,
	pub color: Pixel,
}

impl<'a> Collision {
	pub fn new(ray: Ray, point: V3, raw_normal: V3, t: f32, material: &'a dyn Material) -> Self {
		let front_facing = ray.dir.dot(&raw_normal) < 0f32;
		let normal = if front_facing {raw_normal.clone()} else {raw_normal.clone() * -1f32};
		let (ray_out, color) = material.scatter(&ray, point, normal);
		Collision {
			ray_in: ray,
			ray_out,
			point,
			normal,
			t,
			front_facing,
			color,
		}
	}
}

pub trait Material {
	///returns (reflection, albedo)
	fn scatter(&self, ray_in: &Ray, point: V3, normal: V3) -> (Ray, Pixel);
}

pub struct Diffuse {
	pub albedo: Pixel,
}

impl Material for Diffuse {
	fn scatter(&self, ray_in: &Ray, point: V3, normal: V3) -> (Ray, Pixel) {
		let mut scatter_direction = normal + V3::random_on_unit_sphere();
		//correct some wierdness that might happen when our random offset ~= -normal
		if scatter_direction.near_zero() {
			scatter_direction = normal;
		}

		(Ray {origin: point, dir: scatter_direction}, self.albedo)
	}
}
