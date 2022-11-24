use crate::image_handling::*;
use crate::vectors::*;
use crate::Drawable;
use crate::Camera;

use rand::prelude::*;

pub struct Raytracer {
	
}

impl Raytracer {
	fn get_color(ray: &Ray, scene: &dyn Drawable, depth: u32) -> Pixel {
		if depth <= 0 {
			return Pixel::new();
		}

		match scene.intersect(ray) {
			Some(collision) => {
				Self::get_color(&collision.ray_out, scene, depth - 1).attenuate(&collision.color)
			}
			_ => {
				let unit_direction = ray.dir.normalized();
				let t = 0.5 * (unit_direction.y + 1.0);
				let lerp = |t: f32, start: f32, end: f32| -> f32 {
					start * (1.0 - t) + end * t
				};
				Pixel {
					r: lerp(t, 255.0, 150.0) as u8,
					g: lerp(t, 255.0, 210.0) as u8,
					b: 255,
				}
			}
		}
	}
}

impl Renderer for Raytracer {
    fn render(scene: &dyn Drawable, canvas: &mut dyn Canvas, camera: &Camera) -> Result<(), String> {
		let mut rand = rand::thread_rng();
		let bounds = canvas.bounds();
		let ss_amt = 8;
		for x in 0..bounds.0 {
			for y in 0..bounds.1 {
				let mut pixel = Pixel::new();
				for _ in 0..ss_amt {
					let scaled_x = (x as f32 + rand.gen::<f32>()) / bounds.0 as f32;
					let scaled_y = (y as f32 + rand.gen::<f32>()) / bounds.1 as f32;
	
					let ray = camera.get_ray(scaled_x, scaled_y);
					let color = Self::get_color(&ray, scene, 512);

					pixel = pixel + color.scale(1.0 / ss_amt as f32);
				}
				canvas.put_pixel(x, y, pixel);

			}
		}

		canvas.save()?;

		Ok(())
    }
}
