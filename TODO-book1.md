# Ray Tracing in One Weekend — Coverage Checklist

Verification checklist built from the chapter outline of `books/RayTracingInOneWeekend.html.md`
(v4.0.2). Each item is a feature from the book to confirm in our Rust port. Items marked
**⚠** are places where a quick read turned up a likely deviation from the book; confirm and fix.

## 2. Output an Image ✅
- [x] 2.1 PPM image format: `image_to_ppm` writes P3 header + pixels in row order
      (`src/util/ppm.rs`; tested)
- [x] 2.2 Image file creation: `output.png` via `image` crate (`src/main.rs`); the book's
      gradient test image lives in `ppm::main` (dead code, kept for reference)
- [x] 2.3 Progress indicator: `N / H scanlines done` in `render_frame`
      (fixed: was off by one and used `world.image_height` instead of the render height)

## 3. The vec3 Class ✅
- [x] vec3 ops covered by `Point64` (+, -, unary -, `* f64`, `*=`, `/ f64`) and nalgebra
      (`dot`, `cross`, `magnitude`, `magnitude_squared`, `normalize`); tested in `vector3.rs`
- [x] 3.1 Color utility: `Color64::to_image_rgbu8` averages samples, gamma-corrects, clamps,
      scales to 0–255 (tested)

## 4. Rays, a Simple Camera, and Background ✅
- [x] 4.1 Ray with `at(t)` → `Ray::point_at_parameter` (tested)
- [x] 4.2 Viewport / pixel grid via `Camera`; **added** `Background::SkyGradient` (book's
      white → (0.5, 0.7, 1.0) lerp on `unit_direction.y`) in `src/data/background.rs`.
      `World.background_color: Color64` became `World.background: Background`; existing
      scenes use `Background::Solid(..)`, so their renders are unchanged

## 5. Adding a Sphere ✅
- [x] 5.1 Ray–sphere intersection via discriminant (`src/hittables/sphere.rs`); now skips the
      `sqrt` when the discriminant is negative. Tests cover hit, miss, behind-ray, inside
      (back face), t-range limits, and non-normalized directions
- [x] 5.2 First raytraced image: **added** `World::red_sphere()` (`-w 11`): 400×225,
      vfov 90°, sphere at (0,0,-1) r=0.5, flat red via a non-scattering emitter, sky gradient

## 6. Surface Normals and Multiple Objects ✅
- [x] 6.1 Shading with surface normals: **added** `Material::NormalShaded` (emits
      `0.5 * (n + 1)`, never scatters). `Material::emitted` now takes the `&HitRecord`
- [x] 6.2 Simplified intersection using `h = b/2` (`half_b`)
- [x] 6.3 Hittable abstraction + hit record (`Hittable` enum, `HitRecord` in `src/hittables.rs`)
- [x] 6.4 Front vs. back faces: `HitRecord::new` sets `front_face` and flips the normal to oppose
      the ray (tested for sphere hits from outside and inside)
- [x] 6.5 Hittable list returns the closest hit (`HittableVec`; tested both orderings).
      **Fixed:** `Triangle::is_hit_by` ignored the t-range, so triangles behind the ray (t < 0)
      counted as hits and a farther triangle could override a closer hit in a list
- [x] 6.6 C++ features (shared_ptr/vector): N/A; enums + `Vec`
- [x] 6.7 Constants/utilities: `f64::INFINITY`, `std::f64::consts::PI`, `f64::to_radians`, `rand`
- [x] 6.8 Interval: **added** `src/data/interval.rs` (`EMPTY`, `UNIVERSE`, `size`, `contains`,
      `surrounds`, `clamp`, `with_min`/`with_max`). Every `is_hit_by` now takes
      `ray_t: Interval` instead of `(min, max)`. AABB internals still use raw `t_min/t_max`
      (book 2 converts those)
- [x] Scene: `World::normals_scene()` (`-w 12`): normal-shaded sphere on a ground sphere

## 7. Moving Camera Code Into Its Own Class ✅
- [x] `Camera` (`src/camera.rs`) owns view geometry and `get_ray`; the render loop lives in
      `src/util/render.rs`. Image size, samples and max depth live on `World`; that split is a
      deliberate difference from the book's all-in-one `camera` class

## 8. Antialiasing ✅
- [x] 8.1 Random utilities: `rand` crate (`rng.random::<f64>()`, `random_range`)
- [x] 8.2 Multiple samples per pixel (`-s N`, default 100), summed in `render_frame` and averaged in
      `to_image_rgbu8`. **Fixed:** jitter now maps `(x + rand) / w` instead of `/ (w - 1)`, which
      had stretched the image slightly past the right and top edges of the viewport

## 9. Diffuse Materials
- [ ] 9.1 Simple diffuse (random on hemisphere)
- [ ] 9.2 Max recursion depth limit (`max_depth`, default 50)
- [ ] 9.3 Shadow acne fix: `t_min = 0.001` (`ray.rs:51` — done)
- [ ] 9.4 True Lambertian: `normal + random_unit_vector()` (`lambertian.rs`). Note our
      `random_in_unit_sphere()` actually returns a unit vector *on* the sphere — consider renaming
      to `random_unit_vector`
- [ ] 9.5 Gamma correction (gamma 2, `sqrt`) (`color64.rs` — done)

## 10. Metal
- [ ] 10.1 Material abstraction with `scatter()` (`Material` enum, `src/materials.rs`)
- [ ] 10.2 Hit record carries material
- [ ] 10.3 Light scatter & reflectance (albedo/attenuation; Lambertian degenerate-direction catch)
      — **⚠** `near_zero` (`vector3.rs:44`) doesn't take `abs()`, so any vector with all-negative
      components counts as "near zero"
- [ ] 10.4 Mirrored reflection `reflect(v, n)` (`vector3.rs:48`)
- [ ] 10.5 Metal sphere scene renders
- [ ] 10.6 Fuzzy reflection — **⚠** we check `dot(reflected, n) > 0` *before* adding fuzz; the
      book checks the final fuzzed direction (`metal.rs:18`)

## 11. Dielectrics
- [ ] 11.1–11.2 Refraction / Snell's law `refract()` — **⚠** `vector3.rs:56` uses
      `r_out_normal.magnitude()`; the book uses `length_squared()` (perp/parallel names also swapped)
- [ ] 11.3 Total internal reflection (`cannot_refract`, `dielectric.rs:33` — done)
- [ ] 11.4 Schlick approximation (`reflectance`, `dielectric.rs:9` — done)
- [ ] 11.5 Hollow glass sphere (negative radius / inner sphere with 1/ior) in a scene

## 12. Positionable Camera
- [ ] 12.1 Vertical FOV (`vfov_deg` — done)
- [ ] 12.2 lookfrom / lookat / vup orthonormal basis (`camera.rs:80-82` — done)

## 13. Defocus Blur
- [ ] 13.1 Thin lens: defocus angle/aperture + focus distance (we use `aperture`, book 4.x uses
      `defocus_angle` — fine, just confirm equivalence)
- [ ] 13.2 Sample rays from defocus disk (`random_in_unit_disk`, `get_ray` — done)

## 14. Where Next?
- [ ] 14.1 Final render: random-spheres cover scene (`World::random_world`, `-w 0`) matches book
- [ ] Add regression tests for `refract`, `near_zero`, Schlick, and sphere hit/front-face
