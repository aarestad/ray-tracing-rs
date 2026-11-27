# Ray Tracing: The Next Week — Coverage Checklist

Verification checklist built from the chapter outline of `books/RayTracingTheNextWeek.html.md`
(v4.0.2). Each item is a feature from the book to confirm in our Rust port. Items marked
**⚠** are places where reading the code turned up a likely bug or deviation from the book;
confirm and fix. Scene numbers refer to `-w N` in `src/main.rs`.

## 2. Motion Blur ✅
- [x] 2.1–2.2 Rays carry a time: `Ray::exposure_time` (`src/data/ray.rs`)
- [x] 2.3 Camera samples a random time per ray from its `exposure_time` range (`Camera::get_ray`)
- [x] 2.4 Moving spheres: `MovingSphere::center_at(time)` (`src/hittables/moving_sphere.rs`).
      We store `center0/center1/time0/time1` instead of book 4.x's center ray over t ∈ [0, 1];
      the results are the same. Now skips `sqrt` when the discriminant is negative
- [x] 2.5 Intersection uses the ray's time (tested: same ray misses at t=0, hits at t=1)
- [x] 2.6 Bouncing spheres (`-w 0`). **Fixed** to match the book: *every* diffuse sphere moves
      up by `[0, 0.5)` (was 10% of spheres, moving by up to 1.0); defocus now 0.6° at focus 10
      via `aperture_from_defocus_angle` (was aperture 0.1 ≈ 0.105)

## 3. Bounding Volume Hierarchies ✅
- [x] 3.1–3.2 BVH (`src/hittables/bounded_volume_hierarchy.rs`)
- [x] 3.3 AABB (`src/hittables/axis_aligned_bounding_box.rs`). Still stores min/max points, but
      `is_hit_by` / `hit_interval` now take and return `Interval`. Added `from_corners`
      (any corner order)
- [x] 3.4 Slab test. Zero direction components verified: `±∞` bounds work, and the NaN case
      (origin exactly on a slab plane) is ignored by `f64::max/min`, so that axis places no
      constraint. Documented and tested, including `-0.0`
- [x] 3.5 Bounding boxes for every hittable. **Added** `padded()` (book's `pad_to_minimums`,
      thickness 1e-4 centered on the plane), now used by `Quad` (replacing its own padding code)
      and `Triangle` (previously unpadded)
- [x] 3.6 Boxes of lists (`HittableVec::bounding_box`)
- [x] 3.7 BVH node; ours also caches child boxes and visits the nearer child first. Tested
      against a linear `HittableVec` on 300 random spheres, moving spheres and flat quads ×
      5000 rays: identical closest hits
- [x] 3.8 Splitting at the median; 1- and 2-object cases (tested)
- [x] 3.9 Box comparators (`bvh_comparators.rs`). Still sort by the t=0 box, so moving spheres
      sort by their starting position (fine for correctness)
- [x] 3.10 **Changed** to split on the span's longest axis (`AxisAlignedBoundingBox::longest_axis`)
      instead of a random axis
- [x] Tests: AABB hit/miss/inside/parallel/NaN/padding/longest-axis; BVH ≡ list

## 4. Texture Mapping ✅
- [x] 4.1 Constant color (`Texture::Solid`)
- [x] 4.2 Checker: **switched** to the book-4.x floor-parity checker,
      `Texture::checker(scale, even, odd)` (was the sine-product checker). Scale 0.32 for
      worlds 0, 1 and the teapots (old sine period π/10 ≈ 0.314, so squares are about the same
      size). Handles negative coordinates via `rem_euclid` (tested)
- [x] 4.3 Checker scenes: `-w 1` two checker spheres, `-w 0` checker ground
- [x] 4.4 Sphere UV (`get_sphere_uv`): tested against all six of the book's sample points
- [x] 4.5 Image texture (`src/textures/image.rs`). **Fixed:** pixels are now converted to linear
      light at load (`(c/255)^2.2`, as `stbi_loadf` does), so image textures are no longer
      washed out. Pixels are stored once behind an `Arc` (cloning a texture used to copy the
      whole image). Fixed "cold not" typo. Tests: v flip, clamping, linearization, cyan when
      the file is missing
- [x] 4.6 Earth scene (`-w 5`): camera **moved** to the book's lookfrom (0, 0, 12), vfov 20

## 5. Perlin Noise ✅
- [x] 5.1 Permutation tables + hashing. **Fixed:** shuffle used `random_range(0..i)`
      (Sattolo's algorithm, single-cycle permutations only); now `0..=i` (Fisher–Yates) as in
      the book
- [x] 5.2–5.3 Trilinear + Hermitian smoothing
- [x] 5.4 Frequency scale (`Noise::scale`)
- [x] 5.5 Random lattice vectors. **Fixed:** now unit vectors (were unnormalized
      `rand_range(-1, 1)`). **Fixed:** `trilinear_interp` used the *smoothed* offsets in the
      gradient dot products; now uses the raw `u - i` like the book (smoothed only in blend weights)
- [x] 5.6 Turbulence (7 octaves, `abs`)
- [x] 5.7 Marble phase: `0.5·(1 + sin(scale·z + 10·turb(p)))`
- [x] Scenes `-w 2/3/4` (Perlin / turbulence / marble) render as in the book
- [x] Tests: tables are permutations, unit gradients, zero on lattice points, raw-offset
      interpolation (would fail on the old code), bounded and continuous

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
