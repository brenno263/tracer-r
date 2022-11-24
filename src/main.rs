mod image_handling;
use image_handling::*;

mod vectors;
use vectors::*;

mod drawable;
use drawable::*;

mod sphere;
use sphere::*;

mod camera;
use camera::*;

mod raytracer;
use raytracer::*;

const FILE_NAME: &str = "./out.png";

fn main() {

	let bounds: (usize, usize) = (512, 512);

	let mut image = ImageBuffer::new(bounds.0, bounds.1, FILE_NAME.to_string());

	let fov: f32 = 85.0 * 3.141592654 / 180.0;
	let camera = Camera::new(V3::zero(), V3::z(), V3::y(), fov, bounds.0 as f32 / bounds.1 as f32);

	let diffuse_green = Diffuse { albedo: Pixel::rgb(100, 240, 120) };
	let diffuse_orange = Diffuse { albedo: Pixel::rgb(200, 120, 20) };


	let sphere = Sphere::new(V3::new(0.0, 0.0, 3.0), 0.9, &diffuse_green);
	let sphere2 = Sphere::new(V3::new(2.1, 0.0, 3.0), 1.1, &diffuse_orange);
	let elements: Vec<& dyn Drawable> = vec![&sphere, &sphere2];
	let scene = Scene { elements };

	Raytracer::render(&scene, &mut image, &camera).unwrap();
}
