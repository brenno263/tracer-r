use rand::{thread_rng, Rng};

use crate::image_handling::PixelF;
use crate::ray::Ray;
use crate::utils::lerp;
use crate::vectors::V3;

use serde::{Serialize, Deserialize};

/// A Material defines ways to react to light and propogate color.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Material {
	/// This material reflects in roughly random directions, creating a matte surface.
    Diffuse {
        albedo: PixelF,
    },
	/// This material reflects like a mirror, with optional fuzz to simulate a metallic sheen.
    Specular {
        albedo: PixelF,
        fuzz: f32,
    },
	/// This material refracts and reflects light, like glass or water.
    Dielectric {
        albedo: PixelF,
        r_index_ratio: f32,
        fuzz: f32,
    },
}

impl Material {
    pub fn new_diffuse(albedo: PixelF) -> Self {
        Material::Diffuse { albedo }
    }

    pub fn new_specular(albedo: PixelF, fuzz: f32) -> Self {
        Material::Specular { albedo, fuzz }
    }

    pub fn new_dielectric(albedo: PixelF, r_index: f32, fuzz: f32) -> Self {
        Material::Dielectric {
            albedo,
            r_index_ratio: 1. / r_index,
            fuzz,
        }
    }

    ///returns (reflection, albedo)
    pub fn scatter(&self, ray_in: &Ray, point: V3, normal: V3) -> (Ray, PixelF) {
        match self {
            Material::Diffuse { albedo } => {
                let mut scatter_direction = normal + V3::random_on_unit_sphere();
                //correct some wierdness that might happen when our random offset ~= -normal
                if scatter_direction.near_zero() {
                    scatter_direction = normal;
                }

                (Ray::new(point, scatter_direction), *albedo)
            }
            Material::Specular { albedo, fuzz } => {
                let reflect_direction = Self::reflect(ray_in.dir, normal, *fuzz);

                (Ray::new(point, reflect_direction), *albedo)
            }
            Material::Dielectric {
                albedo,
                r_index_ratio,
                fuzz,
            } => {
                let cos_theta = (ray_in.dir * -1.).dot(&normal);
                let sin_theta = f32::sqrt(1. - (cos_theta * cos_theta));

                let dir = if sin_theta * r_index_ratio > 1.
                    || thread_rng().gen::<f32>() < Self::schlick(cos_theta, *r_index_ratio)
                {
                    // Reflect
                    Self::reflect(ray_in.dir, normal, *fuzz)
                } else {
                    // Refract
                    Self::refract(ray_in.dir, normal, cos_theta, *r_index_ratio)
                };

                (Ray::new(point, dir), *albedo)
            }
        }
    }

    // Helpers

    fn reflect(incoming: V3, normal: V3, fuzz: f32) -> V3 {
        let reflect_direction = incoming - (normal * 2. * incoming.dot(&normal));
        reflect_direction + V3::random_on_unit_sphere() * fuzz
    }

    fn refract(incoming: V3, normal: V3, cos_theta: f32, r_index_ratio: f32) -> V3 {
        let outgoing_perp: V3 = (incoming + (normal * cos_theta)) * r_index_ratio;
        outgoing_perp - (normal * f32::sqrt(1. - outgoing_perp.dot(&outgoing_perp)))
    }

    fn schlick(cos_theta: f32, r_index_ratio: f32) -> f32 {
        let mut t = (1. - r_index_ratio) / (1. + r_index_ratio);
        t = t * t;
        lerp(1., (1. - cos_theta).powi(5), t)
    }
}
