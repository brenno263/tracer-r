# Tracer-R

Tracer-R is a raytracer built by myself - Brennan Seymour - as a final project for MATH 424 - High Performance Computing.
Primarily, this is a subject matter which is pretty cool to me, but it also provides a great example to practice parallel computing, which is a focus of the class.

I have deviated a bit from the rubric by not using MPI to accelerate the program on a high-performance computing cluster.
To make up for that a bit, I've included a binary - tracer-r-mpi - which achieves a similar message-passing pattern using Rust's mpsc channels.
However, it should be noted that this library operates on OS threads, not separate processes like MPI.
As such, it may not have get the same performance increases that MPI does on a high-performance computing cluster.

A write up and presentation can be found in the `presentation` directory, alongside a slew of images and other relevant resources.

## Running Tracer-R

If you don't want to install Rust toolings, I've included some precompiled, statically linked binaries for you.
Check out the `precompiled_binaries` folder for those - you'll most likely want just `tracer-r`.

#### Building the Project

First install `cargo` using the `rustup` installer: [rustup](https://rustup.rs/)

Run the installer:

```bash
$ rustup
```

Build the project:

```bash
# The release flag enables optimizations
$ cargo build --release
```

Then, your output binaries can be found in the `./target` directory.

Alternatively, you can run the project directly through cargo.
Note that you most likely want the `tracer-r` binary.

```bash
$ cargo run --release --bin <benchmark|tracer-r|tracer-r-mpi> -- <arguments>
```

Here's a full example with arguments:

```bash
$ cargo run --release --bin tracer-r -- out.png 128x128 16 bvh grid yes
```
