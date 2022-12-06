
use std::env;
use std::f32::consts::PI;
use std::str::FromStr;
use std::sync::Arc;

use tracer_r::prelude::*;
use tracer_r::*;

enum RtStrategy {
	Naive,
	BVHPointers,
	BVHFlat,
}

impl FromStr for RtStrategy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
			"naive" => Ok(Self::Naive),
			"bvh" => Ok(Self::BVHPointers),
			"bvh_flat" => Ok(Self::BVHFlat),
			_ => Err(()),
		}
    }
}

enum RtScene {
	Sample,
	Grid,
	Random,
}

impl FromStr for RtScene {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
			"sample" => Ok(Self::Sample),
			"grid" => Ok(Self::Grid),
			"random" => Ok(Self::Random),
			_ => Err(()),
		}
    }
}

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 7 {
		eprintln!("Usage: {} FILE <x_pixels>x<y_pixels> <supersampling_amount> <strategy> <scene> <parallel>", &args[0]);
		eprintln!("\tWhere <strategy> is one of: 'naive', 'bvh', 'bvh_flat'");
		eprintln!("\tAnd <scene> is one of: 'sample', 'grid', 'random'");
		eprintln!("\tAnd <parellel> is 'yes' or 'no'");
		return;
	}

	let filename = &args[1];
	let bounds: (usize, usize) = parse_pair(&args[2], 'x').expect("invalid dimensions");
	let ss_amt: usize = usize::from_str(&args[3]).expect("invalid ss_amt");
	let strategy = RtStrategy::from_str(&args[4]).expect("invalid strategy");
	let scene = RtScene::from_str(&args[5]).expect("invalid scene");
	let parallel: bool = &args[6] == "yes";

    let fov: f32 = 70.0 * PI / 180.0;
    let camera = Arc::new(Camera::new(
        V3::new(0., 0., -5.),
        V3::z(),
        V3::y(),
        fov,
        bounds,
    ));

    let elements = match scene {
		RtScene::Sample => sample_scene(),
		RtScene::Grid => big_sphere_grid((14, 14), ((-6., -6.), (6., 6.)), 5.),
		RtScene::Random => random_spheres(256, Bounds { min_point: V3::new(-10., -10., 8.), max_point: V3::new(10., 10., 20.) }),
	};

	let raytracer = Raytracer::default().ss_amt(ss_amt).max_depth(32);

	let image = match strategy {
		RtStrategy::Naive => {
			conditional_render(&raytracer, &camera, &elements, bounds, parallel)
		},
		RtStrategy::BVHPointers => {
			println!("Generating Pointer BVH...");
			let bvh = BVHBuildNode::new(elements, 4);
			println!("Done.");
			conditional_render(&raytracer, &camera, &bvh, bounds, parallel)
		},
		RtStrategy::BVHFlat => {
			println!("Generating Flat BVH...");
			let bvh = BVHBuildNode::new(elements, 4);
			let flat_bvh: LinearBVH = bvh.into();
			println!("Done.");
			conditional_render(&raytracer, &camera, &flat_bvh, bounds, parallel)
		}
	};

    image.save(filename.to_string()).unwrap();
}

