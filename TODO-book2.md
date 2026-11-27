# Ray Tracing: The Next Week — Coverage Checklist

Verification checklist built from the chapter outline of `books/RayTracingTheNextWeek.html.md`
(v4.0.2). Each item is a feature from the book to confirm in our Rust port. Items marked
**⚠** are places where reading the code turned up a likely bug or deviation from the book;
confirm and fix. Scene numbers refer to `-w N` in `src/main.rs`.

## 2. Motion Blur
- [ ] 2.1–2.2 Rays carry a time: `Ray::exposure_time` (`src/data/ray.rs`)
- [ ] 2.3 Camera samples a random time per ray: `Camera::get_ray` draws from `exposure_time`
      range (`src/camera.rs`)
- [ ] 2.4 Moving spheres: `MovingSphere` with `center_at(time)` (`src/hittables/moving_sphere.rs`).
      We store `center0/center1/time0/time1`; book 4.x uses a center *ray* over t ∈ [0, 1].
      Nit: computes `sqrt(discriminant)` before checking it's ≥ 0 (fixed in `Sphere` already)
- [ ] 2.5 Intersection uses the ray's time (`center_at(ray.exposure_time)`)
- [ ] 2.6 Bouncing-spheres scene: `World::random_world` (`-w 0`) — checker ground + 10% moving
      spheres. Confirm camera/defocus match the book's (book: defocus 0.6°, focus 10)

## 3. Bounding Volume Hierarchies
- [ ] 3.1–3.2 BVH concept / hierarchy (`src/hittables/bounded_volume_hierarchy.rs`)
- [ ] 3.3 AABB type (`src/hittables/axis_aligned_bounding_box.rs`). Book 4.x builds it from three
      `Interval`s and adds `aabb::empty` / `aabb::universe`; ours uses min/max points and
      raw `t_min/t_max`. Consider porting to `Interval` (we now have `src/data/interval.rs`)
- [ ] 3.4 Ray–AABB slab test (`hit_interval`). Check the zero-direction-component case
      (`1/0 = ∞`, `0·∞ = NaN`); Rust's `f64::max/min` ignore NaN, so confirm this behaves
- [ ] 3.5 Bounding boxes for every hittable (`bounding_box(time0, time1)` on each variant)
      — **⚠** book 4.x pads *all* AABBs to a minimum thickness (`pad_to_minimums`); we only pad in
      `Quad`. Check `Triangle` and any other flat or degenerate boxes
- [ ] 3.6 Boxes of lists (`HittableVec::bounding_box`)
- [ ] 3.7 BVH node with left/right children; ours also caches child boxes and visits the nearer
      child first. That goes beyond the book, so give it a test
- [ ] 3.8 Splitting: sort by axis, split at the middle; 1- and 2-object cases
- [ ] 3.9 Box comparison functions (`src/hittables/bvh_comparators.rs`). They use
      `bounding_box(0., 0.)`, so moving spheres sort by their t=0 box only
- [ ] 3.10 Another BVH optimization — **⚠** book splits on the *longest axis* of the span's
      bounding box; we pick a random axis (`BOX_COMPARATORS.choose`) (the book's older method)
- [ ] Add tests: AABB hit/miss/inside/parallel-ray; BVH vs `HittableVec` give identical hits

## 4. Texture Mapping
- [ ] 4.1 Constant color texture (`Texture::Solid`, `src/textures/solid_color.rs`)
- [ ] 4.2 Checker texture — **⚠** ours is the *sine-product* checker (book v3) with
      `CHECKER_SCALE = 10`; book 4.x uses `floor(p / scale)` parity with scale 0.32
      (`src/textures.rs`). Decide whether to switch
- [ ] 4.3 Checker scene: `World::two_spheres` (`-w 1`). Book also puts the checker on the
      `random_world` ground (we do)
- [ ] 4.4 Sphere UV coordinates: `get_sphere_uv` (θ = acos(−y), φ = atan2(−z, x) + π) —
      matches book; add a test for the book's sample points (e.g. `<1 0 0>` → `(0.50, 0.50)`)
- [ ] 4.5 Image data access (`src/textures/image.rs`): clamp u,v, flip v, cyan when missing —
      **⚠** book loads images as *linear* floats (`stbi_loadf`, undoing sRGB gamma); we use raw
      sRGB bytes / 255, so image textures come out lighter and washed out after our gamma-2 output.
      Also typo "cold not" in error messages
- [ ] 4.6 Earth scene: `World::earth` (`-w 5`). Book camera: lookfrom (0,0,12), vfov 20

## 5. Perlin Noise
- [ ] 5.1 Blocks of random numbers: permutation tables + hashing (`src/textures/perlin.rs`)
      — **⚠** `permute` uses `random_range(0..i)` (exclusive, Sattolo's algorithm: only single-cycle
      permutations); book uses `random_int(0, i)` (inclusive, Fisher–Yates)
- [ ] 5.2 Trilinear smoothing
- [ ] 5.3 Hermitian smoothing (`u²(3 − 2u)`)
- [ ] 5.4 Frequency scale (`Noise::scale`)
- [ ] 5.5 Random vectors on lattice points — **⚠** book uses *unit* vectors
      (`unit_vector(random(-1,1))`); ours are unnormalized `rand_range(-1, 1)`
      — **⚠** `trilinear_interp` computes `weight_v` from the *smoothed* u,v,w (they shadow the
      raw values); book uses the raw fractional `u - i` in `weight_v` and the smoothed `uu` only in
      the blend weights. Likely produces visible artifacts
- [ ] 5.6 Turbulence (7 octaves, `abs`) (`PerlinGenerator::turbulence`)
- [ ] 5.7 Phase / marble: `0.5·(1 + sin(scale·z + 10·turb(p)))` (`NoiseType::Marble`)
- [ ] Scenes: `-w 2` Perlin, `-w 3` turbulence, `-w 4` marble (`two_perlin_spheres`)
- [ ] Add tests: noise is deterministic for a fixed generator, bounded, continuous across
      lattice cells

## 6. Quadrilaterals
- [ ] 6.1 Quad = Q + u + v (`src/hittables/quad.rs`)
- [ ] 6.2 Ray–plane intersection (parallel rejection, `ray_t.surrounds(t)`)
- [ ] 6.3 Plane from quad (`normal`, `plane_d`)
- [ ] 6.4–6.5 Planar coordinates via `w = n / (n·n)`; α, β
- [ ] 6.6 Interior test `0 ≤ α, β ≤ 1`, UV = (α, β)
- [ ] 6.7 Additional 2D primitives: we have `Triangle` (Möller–Trumbore,
      `src/hittables/triangle.rs`) instead of the book's quad-derived triangle/disk/ellipse.
      Optional: add disk/ellipse
- [ ] Quads scene (book's five-colored quads): **missing**. Consider adding a world
- [ ] Add tests: quad hit/miss at corners/edges, back face, UV values

## 7. Lights
- [ ] 7.1 Emissive material: `DiffuseLight` with `emitted` (`src/materials/diffuse_light.rs`)
- [ ] 7.2 Background color in the ray color function (`Background`, `ray.rs`)
- [ ] 7.3 Objects as lights: `World::simple_light` (`-w 6`) — **⚠** book also adds a *sphere*
      light at (0, 7, 0) r = 2; ours has only the quad light
- [ ] 7.4 Empty Cornell box: walls and light quad match the book (same rectangle as
      `(343,554,332), (-130,0,0), (0,0,-105)`, normal facing down)

## 8. Instances
- [ ] 8.1 Translation (`src/hittables/translation.rs`) — **⚠** BUG: `HitRecord::new(.., &moved_ray, ..)`
      recomputes `location` from the *moved* ray, so the hit point stays in object space (off by
      `-offset`). The book does `rec.p += offset`. Scattered rays from translated objects start
      from the wrong place (Cornell boxes, final-scene sphere cluster, teapots)
- [ ] 8.1 — **⚠** BUG: passes the already-flipped `hr.normal` back into `HitRecord::new`, so
      `front_face` comes out `true` for every hit. Breaks dielectrics inside a `Translation`
- [ ] 8.2 Rotation (`src/hittables/rotation.rs`): any axis via nalgebra (book covers rotate_y
      only); bounding box re-fit over the 8 corners — **⚠** same flipped-normal problem:
      `front_face` always `true`
- [ ] 8.x Cornell box with rotated boxes — **⚠** `World::cornell_box` (`-w 7`) only *translates*
      the boxes; book rotates them 15° and −18° about y (as `cornell_smoke` does)
- [ ] Add tests: translated sphere hit location / normal / front_face; rotated cube hit;
      rotation bounding box

## 9. Volumes
- [ ] 9.1 Constant-density medium (`src/hittables/constant_medium.rs`): two boundary hits,
      clamp to `ray_t` and to 0, exponential free path via `neg_inv_density · ln(rand)`,
      arbitrary normal / front face. Convex boundaries only, as in the book
- [ ] 9.1 Isotropic phase function: `random_unit_vector()` (`src/materials/isotropic.rs`)
- [ ] 9.2 Cornell smoke: `World::cornell_smoke` (`-w 10`): rotated + translated boxes, black and
      white smoke at density 0.01, light (113,554,127)+(330,0,0)+(0,0,305) at 7

## 10. A Scene Testing All New Features (`World::final_scene`, `-w 8`)
- [ ] Ground: 20×20 boxes, random heights 1–101, green (0.48, 0.83, 0.53) in a BVH
- [ ] Light quad (123,554,147), (300,0,0), (0,0,265), intensity 7
- [ ] Moving sphere (400,400,200) → (430,400,200), r 50, (0.7, 0.3, 0.1)
- [ ] Glass sphere (260,150,45) r 50; fuzzy metal (0,150,145) r 50, (0.8,0.8,0.9), fuzz 1
- [ ] Blue subsurface sphere: glass boundary (360,150,145) r 70 + medium 0.2 (0.2, 0.4, 0.9)
- [ ] Global fog: r 5000 boundary, density 0.0001, white
- [ ] Earth (400,200,400) r 100
- [ ] Noise sphere (220,280,300) r 80 — **⚠** book uses `noise_texture(0.2)`, i.e. the *marble*
      formula at scale 0.2; ours is plain `NoiseType::Perlin` at scale 0.1
- [ ] Sphere cluster: 1000 spheres r 10 in [0,165]³ — **⚠** book rotates 15° about y, then
      translates (−100, 270, 395); ours only translates (`// TODO rotation`)
- [ ] Camera: lookfrom (478,278,−600), lookat (278,278,0), vfov 40, 800×800, black background.
      Book max depth is 40; `main.rs` hard-codes 50 for every scene

## Cross-cutting
- [ ] Make `max_depth` a per-world setting (book 2 scenes use 40 or 50)
- [ ] Render each book-2 scene and compare against the book's figures after fixes
