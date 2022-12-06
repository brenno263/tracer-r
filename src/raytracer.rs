use crate::camera::Camera;
use crate::image_handling::PixelF;
use crate::material::Material;
use crate::ray::Ray;
use crate::traits::Drawable;
use crate::traits::{Canvas, Renderer};
use crate::vectors::*;

#[derive(Clone, Debug)]
pub struct Raytracer {
    ss_amt: usize,
    max_depth: usize,
}

impl Raytracer {
    pub fn ss_amt(mut self, ss_amt: usize) -> Self {
        self.ss_amt = ss_amt;
        self
    }

    pub fn max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn get_color(&self, ray: Ray, scene: &dyn Drawable) -> PixelF {
        self.get_color_recursive(ray, scene, 0)
    }

    fn get_color_recursive(&self, ray: Ray, scene: &dyn Drawable, depth: usize) -> PixelF {
        if depth > self.max_depth {
            return PixelF::black();
        }

        match scene.intersect(ray) {
            Some(collision) => self
                .get_color_recursive(collision.ray_out, scene, depth + 1)
                .attenuate(collision.color),
            _ => Self::get_sky_color(ray),
        }
    }

    fn get_sky_color(ray: Ray) -> PixelF {
        let unit_direction = ray.dir.normalized();
        let t = 0.5 * (unit_direction.y + 1.0);
        let lerp = |t: f32, start: f32, end: f32| -> f32 { start * (1.0 - t) + end * t };
        PixelF::rgb_u8(
            lerp(t, 255.0, 120.0) as u8,
            lerp(t, 255.0, 200.0) as u8,
            255,
        )
    }
}

impl Renderer for Raytracer {
    fn render<C: Canvas>(
        &self,
        scene: &dyn Drawable,
        canvas: &mut C,
        camera: &Camera,
    ) -> Result<(), String> {
        let mut rand = rand::thread_rng();
        let bounds = canvas.bounds();
        for x in 0..bounds.0 {
            for y in 0..bounds.1 {
                let mut pixel = PixelF::black();
                for _ in 0..self.ss_amt {
                    let ray = camera.get_ray_perturbed(
                        x + canvas.offset().0,
                        y + canvas.offset().1,
                        &mut rand,
                    );
                    let color = self.get_color(ray, scene);

                    pixel = pixel + color.scale(1.0 / self.ss_amt as f32);
                }
                canvas.put_pixel(x, y, pixel);
            }
        }

        Ok(())
    }
}

impl Default for Raytracer {
    fn default() -> Self {
        Self {
            ss_amt: 8,
            max_depth: 256,
        }
    }
}

#[derive(Clone)]
pub struct Collision {
    pub ray_in: Ray,
    pub ray_out: Ray,
    pub normal: V3,
    pub t: f32,
    pub front_facing: bool,
    pub color: PixelF,
}

impl Collision {
    pub fn new(ray: Ray, raw_normal: V3, t: f32, material: Material) -> Self {
        let front_facing = ray.dir.dot(&raw_normal) < 0f32;
        let normal = if front_facing {
            raw_normal
        } else {
            raw_normal * -1f32
        };
        let (ray_out, color) = material.scatter(&ray, ray.destination(t), normal);
        Collision {
            ray_in: ray,
            ray_out,
            normal,
            t,
            front_facing,
            color,
        }
    }
}
