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

## 9. Diffuse Materials ✅
- [x] 9.1 Simple diffuse: superseded by 9.4 (the book replaces it too)
- [x] 9.2 Child-ray limit: `max_depth` 50 (`main.rs`). We also add Russian roulette after 3
      bounces (`ray.rs`, `RR_MIN_BOUNCES`). That's not in book 1 but is unbiased
      (survivors are re-weighted by `1/p`)
- [x] 9.3 Shadow acne: `Interval::new(0.001, ∞)` in `color_in_world_recurse`
- [x] 9.4 True Lambertian: `normal + random_unit_vector()`. **Renamed** `random_in_unit_sphere`
      → `random_unit_vector` (it always returned a unit vector, which is what the book wants)
- [x] 9.5 Gamma 2 (`sqrt`) in `Color64::to_image_rgbu8` (tested)

## 10. Metal ✅
- [x] 10.1 Material abstraction: `Material` enum with `scatter` / `emitted`
- [x] 10.2 Hit record carries the material
- [x] 10.3 Albedo/attenuation; degenerate scatter direction falls back to the normal.
      **Fixed:** `near_zero` lacked `abs()`, so e.g. `(0, -1, 0)` counted as near-zero (tested)
- [x] 10.4 Mirror reflection `reflect(v, n)` (tested)
- [x] 10.5 Metal spheres render (`-w 13`, `-w 14`)
- [x] 10.6 Fuzzy reflection. **Fixed:** the into-surface check now runs on the *fuzzed*
      direction, and fuzz is capped at 1 like the book's constructor (tested)

## 11. Dielectrics ✅
- [x] 11.1–11.2 Snell's law `refract()`. **Fixed:** used `|r_perp|` instead of `|r_perp|²`, so
      every non-head-on refraction bent at the wrong angle; also clamped `cos θ` before
      negating instead of after (`-(x.min(1))` vs book `min(-x, 1)`), here and in
      `Dielectric::scatter`. The old test checked a value copied from the buggy output; it's
      replaced by a Snell's-law test across angles and η
- [x] 11.3 Total internal reflection (`cannot_refract`; tested)
- [x] 11.4 Schlick approximation (`reflectance`; endpoint values tested)
- [x] 11.5 Hollow glass sphere: **added** `World::book1_materials()` (`-w 13`) with an inner
      n = 1/1.5 bubble inside an n = 1.5 sphere

## 12. Positionable Camera ✅
- [x] 12.1 Vertical FOV (`vfov_deg`)
- [x] 12.2 lookfrom / lookat / vup orthonormal basis (`Camera::new`)

## 13. Defocus Blur ✅
- [x] 13.1 Thin lens: we take `aperture` (lens diameter) where book 4.x takes
      `defocus_angle`; `aperture_from_defocus_angle()` in `worlds.rs` converts
      (`2·focus·tan(angle/2)`). Viewport is placed at `focus_dist`, as in the book
- [x] 13.2 Rays from a uniformly sampled defocus disk (`random_in_unit_disk`, `sqrt(r)`)
- [x] Scene: `-w 13` uses the book's final camera (vfov 20°, defocus 10°, focus 3.4)

## 14. Where Next? ✅
- [x] 14.1 Final render: **added** `World::book1_final()` (`-w 14`, BVH unless `-f`): gray
      ground, static random spheres, sky gradient, 1200×675, defocus 0.6°, focus 10. World 0
      remains the book-2 variant (checker ground, moving spheres); both share
      `add_random_small_spheres()`

## Remaining differences from the book (intentional)
- Russian roulette path termination (book 3 territory)
- `Camera` / `World` split instead of one all-in-one `camera` class
- PNG output by default; PPM writer kept in `src/util/ppm.rs`
- Color quantization uses `255 · clamp(x, 0, 1)` rather than `256 · clamp(x, 0, 0.999)`
