use std::f32::consts::PI;
use std::time::Instant;

use tracer_r::prelude::*;
use tracer_r::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <number_of_spheres>", &args[0]);
    }

    let num_spheres = usize::from_str_radix(&args[1], 10)
        .or(Err(&args[1]))
        .expect("invalid number of spheres");

    let (r, c, e, b) = setup(num_spheres);

    timed_run("naive", || {
        let i = render(&r, &c, &e, b);
        i.save("dump.png".to_owned()).unwrap();
    });

    timed_run("bvh", || {
        let bvh = BVHBuildNode::new(e.clone(), 4);
        let i = render(&r, &c, &bvh, b);
        i.save("dump.png".to_owned()).unwrap();
    });

    timed_run("flat bvh", || {
        let bvh = BVHBuildNode::new(e.clone(), 4);
        let fbvh: LinearBVH = bvh.into();
        let i = render(&r, &c, &fbvh, b);
        i.save("dump.png".to_owned()).unwrap();
    });

    timed_run("naive parallel", || {
        let i = par_render(&r, &c, &e, b);
        i.save("dump.png".to_owned()).unwrap();
    });

    timed_run("bvh parallel", || {
        let bvh = BVHBuildNode::new(e.clone(), 4);
        let i = par_render(&r, &c, &bvh, b);
        i.save("dump.png".to_owned()).unwrap();
    });

    timed_run("flat bvh parallel", || {
        let bvh = BVHBuildNode::new(e.clone(), 4);
        let fbvh: LinearBVH = bvh.into();
        let i = par_render(&r, &c, &fbvh, b);
        i.save("dump.png".to_owned()).unwrap();
    });
}

fn setup(num_spheres: usize) -> (Raytracer, Camera, Vec<Primitive>, (usize, usize)) {
    let bounds = (256, 256);

    let rt = Raytracer::default().ss_amt(16).max_depth(32);

    let fov: f32 = 70.0 * PI / 180.0;
    let camera = Camera::new(V3::new(0., 0., -5.), V3::z(), V3::y(), fov, bounds);

    let elements = random_spheres(
        num_spheres,
        Bounds {
            min_point: V3::new(-10., -10., 10.),
            max_point: V3::new(10., 10., 24.),
        },
    );

    (rt, camera, elements, bounds)
}

fn timed_run<F>(description: &str, f: F)
where
    F: Fn() -> (),
{
    let now = Instant::now();
    for _ in 0..5 {
        f();
    }
    let elapsed = now.elapsed();
    println!(
        "Ran {} in {:.3} seconds",
        description,
        elapsed.as_secs_f32() / 5.
    );
}
