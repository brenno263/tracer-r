use rand::rngs::ThreadRng;
use rand::Rng;

use crate::ray::Ray;
use crate::vectors::V3;

/// The camera controls our point of view. It is used to cast rays into the scene.
#[derive(Debug)]
pub struct Camera {
    position: V3,
    // position of the upper-left corner of the viewport relative to position
    upper_left: V3,
    // vector from left to right viewport border
    horizontal: V3,
    // vector from bottom to top viewport border
    vertical: V3,
    /// Our viewport bounds in pixels
    bounds: (usize, usize),
}

impl Camera {
    pub fn new(position: V3, direction: V3, up: V3, fov: f32, bounds: (usize, usize)) -> Camera {
        let aspect_r = bounds.0 as f32 / bounds.1 as f32;
        let z = direction.normalized();
        let y = up.normalized();
        let x = z.cross(&y);
        let vertical = y * (2.0 * f32::tan(fov / 2.0));
        let horizontal = x * (-2.0 * f32::tan(fov / 2.0) * aspect_r);
        let upper_left = position + z - (horizontal * 0.5) + (vertical * 0.5);
        Camera {
            position,
            upper_left,
            horizontal,
            vertical,
            bounds,
        }
    }

	/// Get a ray coming out of the camera at these pixel coordinates.
    pub fn get_ray(&self, x: usize, y: usize) -> Ray {
        let x_frac = x as f32 / self.bounds.0 as f32;
        let y_frac = y as f32 / self.bounds.1 as f32;
        self.get_ray_from_f32(x_frac, y_frac)
    }

	// Get a ray coming out of the camera at these pixel coordinates, with sub-pixel perturbation for supersampling.
    pub fn get_ray_perturbed(&self, x: usize, y: usize, rand: &mut ThreadRng) -> Ray {
        let x_frac = (x as f32 + rand.gen::<f32>()) / self.bounds.0 as f32;
        let y_frac = (y as f32 + rand.gen::<f32>()) / self.bounds.1 as f32;
        self.get_ray_from_f32(x_frac, y_frac)
    }

    /// takes x, y in [0, 1)x[0, 1)
    pub fn get_ray_from_f32(self: &Self, x: f32, y: f32) -> Ray {
        let dir = self.upper_left + (self.horizontal * x) - (self.vertical * y) - self.position;
        Ray::new(self.position, dir.normalized())
    }
}
