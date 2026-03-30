# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build --release      # Optimized build (use for render runs)
cargo test                 # Run all tests
cargo fmt                  # Format code
cargo clippy               # Lint
cargo run -- -h            # Show CLI help
cargo run -- -w <0-8> -b   # Render scene (world 0-8), -b enables BVH
```

## Architecture

This is a ray tracer implementing the [Ray Tracing in One Weekend](https://raytracing.github.io/) series in Rust. The books are located locally in the `books` directory for reference. The reference implementation is in C++, but we have translated the book's code into Rust.

### Enum Dispatch Design

All polymorphic types use **enums instead of trait objects** for performance. There are three core enums:
- `Hittable` (`hittables.rs`) — wraps Sphere, MovingSphere, Quad, Cuboid, Triangle, Translation, Rotation, HittableVec, BVH
- `Material` (`materials/materials.rs`) — wraps Lambertian, Metal, Dielectric, DiffuseLight
- `Texture` (`textures/textures.rs`) — wraps SolidColor, Checker, Noise, Image

When adding a new hittable/material/texture, add a variant to the relevant enum and implement the dispatch arm.

### Rendering Pipeline

`main.rs` → parse args → select world from `util/worlds.rs` → build scene (camera + hittables + materials) → threadpool processes pixels in parallel → each pixel shoots N rays → `Ray::color_in_world()` recurses up to depth 50 → collect via channel → save as `output.png`.

### Key Types

- `Point64` (`data/point64.rs`) — wraps `nalgebra::Vector3<f64>`, used for both positions and direction vectors
- `Vector3` (`data/vector3.rs`) — utility functions over `nalgebra::Vector3<f64>`
- `Color64` (`data/color64.rs`) — RGB color (f64 components)
- `Ray` (`data/ray.rs`) — origin + direction, contains the recursive `color_in_world()` method

### Scenes

8 pre-built scenes in `util/worlds.rs` (selectable via `-w 0..8`): random spheres, checkerboard, Perlin noise variations, Earth texture, Cornell box, and a complex final scene. The `-b` flag wraps the hittable list in a `BoundedVolumeHierarchy` for acceleration.
