use std::f32::consts::PI;
use std::sync::{mpsc, Arc};
use std::thread;

use tracer_r::prelude::*;
use tracer_r::*;

fn main() {

	let args: Vec<String> = std::env::args().collect();
	if args.len() != 2 {
		eprintln!("usage: {} <filename>", &args[0]);
		return;
	}

    let bounds = (512, 512);

    let rt = Arc::new(Raytracer::default().ss_amt(8).max_depth(32));

    let fov: f32 = 70.0 * PI / 180.0;
    let camera = Arc::new(Camera::new(
        V3::new(0., 0., -5.),
        V3::z(),
        V3::y(),
        fov,
        bounds,
    ));

    let elements = big_sphere_grid((14, 14), ((-6., -6.), (6., 6.)), 5.);
    let bvh = Arc::new(BVHBuildNode::new(elements, 4));

    let chunks = ImageBuffer::bands(bounds, 32);

    // We process senders in a scope block so that the root sender gets dropped at the end.
    // We can move out our receiver and handles, which are all we really need now.
    let (receiver, handles) = {
        let (sender, receiver) = mpsc::channel();

        let mut handles = Vec::new();

        for mut chunk in chunks {
            let child_sender = sender.clone();
            let child_rt = rt.clone();
            let child_camera = camera.clone();
            let child_bvh = bvh.clone();
            handles.push(thread::spawn(move || {
                child_rt
                    .render(&*child_bvh, &mut chunk, &child_camera)
                    .unwrap();
                println!("Sending!!");
                child_sender.send(chunk).unwrap();
            }));
        }

        (receiver, handles)
    };

    // Iterate over the reciever, blocking between values recieved.
    // When all senders have been closed, the reciever will as well.
    let mut processed_chunks: Vec<ImageBuffer> = receiver.iter().collect();

    // Sort our recieved chunks by y-offset, so they're not jumbled.
    processed_chunks.sort_by_key(|c| c.offset.1);

    // Concatenate the chunks into an image, then write it.
    let mut image = ImageBuffer::new(bounds.0, 0);
    for mut chunk in processed_chunks {
        image.append_rows(&mut chunk);
    }

	// Our threads should really be done if all the senders are dropped.
	// However, we can join then anyways - it may be useful to propogate a thread panic.
    for h in handles {
        h.join().unwrap();
    }

	image.save(args[1].clone()).unwrap();
}
