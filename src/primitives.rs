use crate::{
    bounded_volume_hierarchy::Bounds,
    material::Material,
    ray::Ray,
    raytracer::Collision,
    traits::{Boundable, Drawable},
    vectors::*,
};


/// This represents a primitive object which can be rendered.
/// It's an enum to leave room for triangles, quads, meshes, etc.
/// Those don't exist in the project yet, but I may add them later.
#[derive(Clone, Debug)]
pub enum Primitive {
    Sphere {
        center: V3,
        radius: f32,
        material: Material,
    },
}

impl Primitive {
    pub fn new_sphere(center: V3, radius: f32, material: Material) -> Self {
        Primitive::Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Drawable for Primitive {
    fn intersect(self: &Self, ray: Ray) -> Option<Collision> {
        match *self {
            Primitive::Sphere {
                center,
                radius,
                material,
            } => {
                //t^2(D*D) + 2t(D*(O-C)) + (O-C) * (O-C) - r^2 = 0
                let center_to_ray_origin: V3 = ray.origin - center;
                let a = ray.dir.dot(&ray.dir);
                let half_b = ray.dir.dot(&center_to_ray_origin);
                let c = center_to_ray_origin.dot(&center_to_ray_origin) - (radius * radius);

                let discriminant = half_b * half_b - a * c;
                if discriminant < 0. {
                    return None;
                }

                let sqrtd = discriminant.sqrt();

                //get the closer root
                let mut root = (-half_b - sqrtd) / a;

                if root < ray.min || root > ray.max {
                    root = (-half_b + sqrtd) / a;
                    if root < ray.min || root > ray.max {
                        return None;
                    }
                }

                let point = ray.destination(root);
                let raw_normal = (point - center) / radius;
                let faced_normal = ray.get_faced_normal(raw_normal);

                Option::Some(Collision::new(ray, faced_normal, root, material))
            }
        }
    }
}

impl Boundable for Primitive {
    fn bounds(&self) -> Bounds {
        match *self {
            Primitive::Sphere {
                center,
                radius,
                material: _,
            } => {
                let radius_offset = V3::new(radius, radius, radius);
                Bounds {
                    min_point: center - radius_offset,
                    max_point: center + radius_offset,
                }
            }
        }
    }
}

impl Drawable for Vec<Primitive> {
    fn intersect(&self, mut ray: Ray) -> Option<Collision> {
        let mut out = None;
        for ref el in self {
            if let Some(coll) = el.intersect(ray) {
                ray.max = coll.t;
                out = Some(coll);
            }
        }
        out
    }
}
