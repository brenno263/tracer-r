#![allow(dead_code)]

mod image_handling;
mod traits;

mod bounded_volume_hierarchy;
mod camera;
mod material;
mod primitives;
// mod partitionable;
mod ray;
mod raytracer;
mod utils;
mod vectors;

pub mod prelude;
use prelude::*;
use rand::Rng;
use rayon::prelude::*;

pub fn conditional_render<S>(rt: &Raytracer, camera: &Camera, scene: &S, bounds: (usize, usize), parallel: bool) -> ImageBuffer
	where S: Drawable + Send + Sync
{
	if parallel {
		par_render(rt, camera, scene, bounds)
	} else {
		render(rt, camera, scene, bounds)
	}
}

pub fn render<S>(rt: &Raytracer, camera: &Camera, scene: &S, bounds: (usize, usize)) -> ImageBuffer
	where S: Drawable
{
	let mut image_out = ImageBuffer::new(bounds.0, bounds.1);
	rt.render(scene, &mut image_out, camera).unwrap();
	image_out
}

pub fn par_render<S>(rt: &Raytracer, camera: &Camera, scene: &S, bounds: (usize, usize)) -> ImageBuffer
	where S: Drawable + Send + Sync
{
	let mut chunks = ImageBuffer::bands(bounds, 16);
	chunks.par_iter_mut().for_each(|chunk| {
		rt.render(scene, chunk, camera).unwrap();
	});

	let mut image_out = ImageBuffer::new(bounds.0, 0);
	for mut chunk in chunks {
		image_out.append_rows(&mut chunk);
	}
	image_out
}

pub fn random_spheres(num: usize, bounds: Bounds) -> Vec<Primitive> {
	let mut rand = rand::thread_rng();
	let mut elements: Vec<Primitive> = Vec::with_capacity(num);

	for _ in 0..num {
		let x: f32 = rand.gen_range(bounds.min_point.x..bounds.max_point.x);
		let y: f32 = rand.gen_range(bounds.min_point.y..bounds.max_point.y);
		let z: f32 = rand.gen_range(bounds.min_point.z..bounds.max_point.z);
		let color = PixelF::random();
		let param: f32 = rand.gen();
		let radius: f32 = rand.gen::<f32>() + 0.5;

		let mat_pick: usize = rand.gen_range(0..3);
		let mat = match mat_pick {
			0 => Material::new_diffuse(color),
			1 => Material::new_specular(color, param),
			_ => Material::new_dielectric(color, 1. + param * param, 0.005),
		};

		elements.push(Primitive::new_sphere(
			V3::new(x, y, z),
			radius,
			mat
		));
	}

	elements
}

pub fn big_sphere_grid(
    grid_dims: (usize, usize),
    world_dims: ((f32, f32), (f32, f32)),
    z: f32,
) -> Vec<Primitive> {
    let mut rand = rand::thread_rng();
    let mut elements: Vec<Primitive> = Vec::with_capacity(grid_dims.0 * grid_dims.1);

    for y in 0..grid_dims.1 {
        for x in 0..grid_dims.0 {
            let x_t = x as f32 / grid_dims.0 as f32;
            let y_t = y as f32 / grid_dims.1 as f32;

            let color = PixelF::random();
            let mat = if rand.gen_bool(0.5) {
                Material::new_diffuse(color)
            } else {
                Material::new_specular(color, 0.1)
            };

            let sphere = Primitive::new_sphere(
                V3::new(
                    lerp(world_dims.0 .0, world_dims.1 .0, x_t) - 0.5,
                    lerp(world_dims.0 .1, world_dims.1 .1, y_t) - 0.5,
                    z + rand.gen::<f32>(),
                ),
                0.5,
                mat,
            );
            elements.push(sphere);
        }
    }
    elements
}

pub fn sample_scene() -> Vec<Primitive> {
    let diffuse_orange = Material::new_diffuse(PixelF::rgb_u8(200, 120, 30));
    let diffuse_dark_blue = Material::new_diffuse(PixelF::rgb(0.08, 0.1, 0.4));
    let specular_gold = Material::new_specular(PixelF::rgb(1., 0.8, 0.4), 0.2);
    let specular_red = Material::new_specular(PixelF::rgb(0.8, 0.2, 0.3), 0.);
    let specular_mirror = Material::new_specular(PixelF::rgb(0.9, 0.8, 1.), 0.05);
    let dielectric_teal = Material::new_dielectric(PixelF::rgb(0.5, 0.8, 1.), 1.16, 0.);

    let sphere  = Primitive::new_sphere(V3::new(0.0, 0.0, 0.), 0.9, specular_gold);
    let sphere2 = Primitive::new_sphere(V3::new(2.1, 0.0, 0.), 1.1, diffuse_orange);
    let sphere3 = Primitive::new_sphere(V3::new(-1.9, 0.3, 0.), 0.9, diffuse_dark_blue);
    let sphere4 = Primitive::new_sphere(V3::new(0.3, 0.3, -2.), 0.6, dielectric_teal);
    let sphere5 = Primitive::new_sphere(V3::new(0., -100.8, 0.), 100., specular_mirror);
    let sphere6 = Primitive::new_sphere(V3::new(-2.3, 3.2, 3.3), 2.2, specular_red);
    vec![sphere, sphere2, sphere3, sphere4, sphere5, sphere6]
}
