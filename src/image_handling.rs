use image;
use crate::{Drawable, camera::Camera};

/// relatively generic way of using canvases, so that we can adapt to use a variety of output methods.
pub trait Canvas {
	fn put_pixel(self: &mut Self, x: usize, y: usize, pixel: PixelF);
	fn save(self: &Self) -> Result<(), String>;
	fn bounds(self: &Self) -> (usize, usize);
}

pub trait Renderer {
	fn render(scene: &dyn Drawable, canvas: &mut dyn Canvas, camera: &Camera) -> Result<(), String>;
}

pub struct ImageBuffer {
	pub width: usize,
	pub height: usize,
	pub filename: String,
	pub pixels: Vec<PixelF>,
}

impl ImageBuffer {
	pub fn new(width: usize, height: usize, filename: String) -> Self {
		ImageBuffer {width, height, filename, pixels: vec![PixelF::black(); width * height]}
	}

	pub fn to_bytes(self: &Self) -> Vec<u8> {
		let mut bytes = Vec::<u8>::with_capacity(self.width * self.height * 3);
		for px in &self.pixels {
			bytes.extend_from_slice(&px.to_bytes());
		}
		bytes
	}
}

impl Canvas for ImageBuffer {
	fn put_pixel(self: &mut Self, x: usize, y: usize, pixel: PixelF) {
		self.pixels[y * self.width + x] = pixel;
	}

	fn save(self: &Self) -> Result<(), String> {
		match image::save_buffer(
			&self.filename, 
			&self.to_bytes(), 
			self.width as u32, 
			self.height as u32, 
			image::ColorType::Rgb8
		) {
			Err(error) => {
				Err(error.to_string())
			}
			Ok(()) => Ok(())
		}
	}

	fn bounds(self: &Self) -> (usize, usize) {(self.width, self.height)}
}

#[derive(Clone, Copy)]
pub struct PixelF {
	pub r: f32,
	pub g: f32,
	pub b: f32,
}

impl PixelF {
	pub fn black() -> Self {
		Self { r: 0.0, g: 0.0, b: 0.0 }
	}

	pub fn rgb(r: f32, g: f32, b: f32) -> Self {
		Self {r, g, b}
	}

	pub fn rgb_u8(r: u8, g: u8, b: u8) -> Self {
		Self::rgb(
			Self::color_u8_to_f32(r),
			Self::color_u8_to_f32(g),
			Self::color_u8_to_f32(b),
		)
	}

	pub fn color_u8_to_f32(u: u8) -> f32 {
		u as f32 / 255.
	}

	pub fn color_f32_to_u8(f: f32) -> u8 {
		(f * 255.) as u8
	}

	pub fn attenuate(self, other: Self) -> Self {
		Self::rgb(
			self.r * other.r,
			self.g * other.g,
			self.b * other.b,
		)
	}

	pub fn scale(self, scalar: f32) -> Self {
		Self::rgb(
			(self.r * scalar).clamp(0., 1.),
			(self.g * scalar).clamp(0., 1.),
			(self.b * scalar).clamp(0., 1.),
		)
	}

	pub fn to_bytes(self) -> [u8;3] {
		[
			Self::color_f32_to_u8(self.r),
			Self::color_f32_to_u8(self.g),
			Self::color_f32_to_u8(self.b),
		]
	}
}



impl std::ops::Add for PixelF {
    type Output = PixelF;

    fn add(self, rhs: Self) -> Self::Output {
        Self::rgb(
			(self.r + rhs.r).clamp(0., 1.),
			(self.g + rhs.g).clamp(0., 1.),
			(self.b + rhs.b).clamp(0., 1.),
		)
    }
}

impl std::fmt::Display for PixelF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.r, self.g, self.b)
    }
}
