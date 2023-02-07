use image;
use rand::Rng;

use crate::traits::Canvas;

use serde::{Serialize, Deserialize};


/// Represents an image buffer than can be written to, then saved to a file.
/// This is what the raytracer writes to, abstacting away file shenanigans.
#[derive(Debug)]
pub struct ImageBuffer {
    pub bounds: (usize, usize),
    pub offset: (usize, usize),
    pub pixels: Vec<PixelF>,
}

impl ImageBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        ImageBuffer {
            bounds: (width, height),
            offset: (0, 0),
            pixels: vec![PixelF::black(); width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.bounds.0
    }

    pub fn height(&self) -> usize {
        self.bounds.1
    }

	/// Create a number of ImageBuffers with offsets to represent a number of bands in an image.
	/// Great for parallelizing, as you can then append these bands onto an empty ImageBuffer to
	/// reassemble an image.
    pub fn bands(bounds: (usize, usize), rows_per_band: usize) -> Vec<ImageBuffer> {
        let remainder_band_rows = bounds.1 % rows_per_band;
        let num_bands = bounds.1 / rows_per_band + if remainder_band_rows > 0 { 1 } else { 0 };
        let mut chunks = Vec::with_capacity(num_bands);

        for b in 0..num_bands {
            let is_remainder_band = remainder_band_rows > 0 && b == num_bands - 1;

            chunks.push(ImageBuffer {
                bounds: (
                    bounds.0,
                    if is_remainder_band {
                        remainder_band_rows
                    } else {
                        rows_per_band
                    },
                ),
                offset: (0, b * rows_per_band),
                pixels: vec![
                    PixelF::black();
                    bounds.0
                        * if is_remainder_band {
                            remainder_band_rows
                        } else {
                            rows_per_band
                        }
                ],
            })
        }

        chunks
    }

	/// This creates bands from an existing image. These bands can be edited in-place, and don't need to be reassembled.
	/// However, Rust cannot guarantee safety when passing references like this between threads, so this cannot be used
	/// with conventional multithreading.
    pub fn bands_in_place(&mut self, rows_per_band: usize) -> Vec<InPlaceSubBuffer<'_>> {
        let remainder_band_rows = self.bounds.1 % rows_per_band;
        let num_bands = self.bounds.1 / rows_per_band + if remainder_band_rows > 0 { 1 } else { 0 };
        let mut chunks = Vec::with_capacity(num_bands);

        for (b, chunk) in self
            .pixels
            .chunks_mut(self.bounds.0 * rows_per_band)
            .enumerate()
        {
            let is_remainder_band = remainder_band_rows > 0 && b == num_bands - 1;

            chunks.push(InPlaceSubBuffer {
                bounds: (
                    self.bounds.0,
                    if is_remainder_band {
                        remainder_band_rows
                    } else {
                        rows_per_band
                    },
                ),
                offset: (0, b * rows_per_band),
                pixels: chunk,
            })
        }

        chunks
    }

    pub fn append_rows(&mut self, other: &mut ImageBuffer) {
        assert!(other.offset.1 == self.bounds.1);
        self.pixels.append(&mut other.pixels);
        self.bounds.1 += other.bounds.1;
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::with_capacity(self.bounds.0 * self.bounds.1 * 3);
        for px in &self.pixels {
            bytes.extend_from_slice(&px.to_bytes());
        }
        bytes
    }

    pub fn save(&self, filename: String) -> Result<(), String> {
        let (width, height) = self.bounds;
        match image::save_buffer(
            filename,
            &self.to_bytes(),
            width as u32,
            height as u32,
            image::ColorType::Rgb8,
        ) {
            Err(error) => Err(error.to_string()),
            Ok(()) => Ok(()),
        }
    }
}

impl Canvas for ImageBuffer {
    fn put_pixel(self: &mut Self, x: usize, y: usize, pixel: PixelF) {
        self.pixels[y * self.bounds.0 + x] = pixel;
    }

    fn bounds(self: &Self) -> (usize, usize) {
        self.bounds
    }

    fn offset(&self) -> (usize, usize) {
        self.offset
    }
}

#[derive(Debug)]
pub struct InPlaceSubBuffer<'parent> {
    bounds: (usize, usize),
    offset: (usize, usize),
    pixels: &'parent mut [PixelF],
}

impl<'parent> Canvas for InPlaceSubBuffer<'parent> {
    fn put_pixel(&mut self, x: usize, y: usize, pixel: PixelF) {
        self.pixels[y * self.bounds.0 + x] = pixel;
    }

    fn bounds(&self) -> (usize, usize) {
        self.bounds
    }

    fn offset(&self) -> (usize, usize) {
        self.offset
    }
}

/// PixelF represents a single pixel whose r, g, and b values are f32s in [0, 1]
/// These are used in processing, since they have high accuracy, and are then
/// converted to u8s for export to file.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PixelF {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl PixelF {
    pub fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
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
        Self::rgb(self.r * other.r, self.g * other.g, self.b * other.b)
    }

    pub fn scale(self, scalar: f32) -> Self {
        Self::rgb(
            (self.r * scalar).clamp(0., 1.),
            (self.g * scalar).clamp(0., 1.),
            (self.b * scalar).clamp(0., 1.),
        )
    }

    pub fn to_bytes(self) -> [u8; 3] {
        [
            Self::color_f32_to_u8(self.r),
            Self::color_f32_to_u8(self.g),
            Self::color_f32_to_u8(self.b),
        ]
    }

    pub fn random() -> Self {
        let mut r = rand::thread_rng();
        Self::rgb(r.gen(), r.gen(), r.gen())
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
