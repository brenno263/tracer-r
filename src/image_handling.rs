use image;
use crate::{Drawable, camera::Camera};

/// relatively generic way of using canvases, so that we can adapt to use a variety of output methods.
pub trait Canvas {
	fn put_pixel(self: &mut Self, x: usize, y: usize, pixel: Pixel);
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
	pub pixels: Vec<Pixel>,
}

impl ImageBuffer {
	pub fn new(width: usize, height: usize, filename: String) -> Self {
		ImageBuffer {width, height, filename, pixels: vec![Pixel::new(); width * height]}
	}

	pub fn to_bytes(self: &Self) -> Vec<u8> {
		let mut bytes = Vec::<u8>::with_capacity(self.width * self.height * 3);
		for px in &self.pixels {
			bytes.extend_from_slice(&px.to_byte_array());
		}
		bytes
	}
}

impl Canvas for ImageBuffer {
	fn put_pixel(self: &mut Self, x: usize, y: usize, pixel: Pixel) {
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
pub struct Pixel {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

impl Pixel {
	pub fn new() -> Self {
		Pixel {r: 0, g: 0, b: 0}
	}

	pub fn rgb(r: u8, g: u8, b: u8) -> Self {
		Pixel {r, g, b}
	}

	pub fn attenuate(self: &Self, other: &Self) -> Pixel {
		let attn = |m: u8, n: u8| -> u8 {
			// very slightly more performant than m * n / 255
			((m as u16 * n as u16 + 255) >> 8) as u8
		};

		Pixel {
			r: attn(self.r, other.r),
			g: attn(self.g, other.g),
			b: attn(self.b, other.b),
		}
	}

	pub fn scale(self: &Self, scalar: f32) -> Pixel {
		let scale = |x: u8, s: f32| -> u8 {
			(x as f32 * s).clamp(0.0, 255.0) as u8
		};

		Pixel {
			r: scale(self.r, scalar),
			g: scale(self.g, scalar),
			b: scale(self.b, scalar)
		}
	}

	pub fn to_byte_array(self: &Self) -> [u8;3] {
		[self.r, self.g, self.b]
	}
}

impl std::ops::Add for Pixel {
    type Output = Pixel;

    fn add(self, rhs: Self) -> Self::Output {
        Pixel {
			r: self.r.saturating_add(rhs.r),
			g: self.g.saturating_add(rhs.g),
			b: self.b.saturating_add(rhs.b),
		}
    }
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.r, self.g, self.b)
    }
}
