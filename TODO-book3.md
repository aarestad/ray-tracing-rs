# Ray Tracing: The Rest of Your Life — Coverage Checklist

Verification checklist built from the chapter outline of `books/RayTracingTheRestOfYourLife.html.md`
(v4.0.2). Unlike books 1 and 2, **almost none of book 3 exists yet**: there are no PDFs, no ONB, no
light sampling and no stratified pixel sampling. The only book-3-style feature is Russian
roulette path termination (`src/data/ray.rs`, `RR_MIN_BOUNCES`), which isn't in the book. Most
items below are therefore *implement* tasks rather than *verify* tasks. Items marked **⚠** are
design decisions or interactions with existing code to settle first.

## Design notes (Rust adaptation)
- The book uses virtual classes (`pdf`, `hittable::pdf_value/random`, `material::scattering_pdf`).
  Per `CLAUDE.md`, use **enums**: e.g. `enum Pdf { Sphere, Cosine(Onb), Hittable { .. },
  Mixture(Box<Pdf>, Box<Pdf>) }` with `value(direction)` / `generate()` dispatch arms.
  `Pdf::Hittable` needs a reference to a hittable plus an origin: use a lifetime
  (`Pdf<'a>`) or an `Arc<Hittable>` for the lights list
- `ScatterRecord` (`src/materials.rs`) gains the book's `pdf` / `skip_pdf` / `skip_pdf_ray`.
  It currently carries `hit_record` (unused, `#[allow(dead_code)]`) and `scattered`
- **⚠** `Ray::color_in_world` needs a *lights* hittable (book's `ray_color(r, depth, world,
  lights)`). Thread it through `render_frame` / `render_row` and `World` (new `lights` field),
  and decide what interactive mode does (`src/util/interactive.rs`)
- **⚠** Russian roulette: keep it, gate it, or drop it. It's unbiased, but it changes the noise
  you'd compare against the book's figures. Consider a flag, or apply it only after N bounces
  as today

## 2. A Simple Monte Carlo Program
- [ ] 2.1 Estimating π (`pi.cc`): standalone program. Suggest `examples/pi.rs`
      (`cargo run --example pi`) to keep it out of the renderer
- [ ] 2.2 Showing convergence (running estimate printed periodically)
- [ ] 2.3 Stratified samples (jittering): π estimate with a √N × √N grid vs. random, showing
      stratification's lower error
- [ ] 2.3 **Renderer:** stratify samples *inside each pixel*: `sqrt_spp` grid with
      `recip_sqrt_spp` jitter (`camera::get_ray(i, j, s_i, s_j)`, `sample_square_stratified`).
      **⚠** `render_frame` runs `num_samples` whole-row passes calling `render_row`; pass the
      sample index so each pass picks its sub-cell, and round `-s` to a perfect square (book does
      `sqrt_spp = int(sqrt(samples_per_pixel))`)
- [ ] 2.3 Cornell box revisited with stratification (book's comparison image)

## 3. One-Dimensional Monte Carlo Integration
- [ ] 3.1–3.2 Expected value; integrate x² over [0, 2] (`integrate_x_sq.cc`), plus the
      `sin⁵` and `ln(sin)` examples. Suggest `examples/integrate_x_sq.rs`
- [ ] 3.3–3.5 Density functions, constructing a PDF, choosing samples via the inverse CDF
      (`estimate_halfway.cc`: estimating the 50% point)
- [ ] 3.6 Approximating distributions (crude first-order approximation of a non-uniform PDF)
- [ ] 3.7 Importance sampling x² with uniform, linear and quadratic (perfect) PDFs; show the
      variance drop and the "perfect PDF needs only 1 sample" result

## 4. Monte Carlo Integration on the Sphere of Directions
- [ ] Importance-sampled integral of `cos²θ` over the unit sphere (`sphere_importance.cc`),
      using `random_unit_vector()` with PDF `1/(4π)`. Suggest `examples/sphere_importance.rs`

## 5. Light Scattering
- [ ] 5.1–5.2 Albedo and scattering as a probability (concepts)
- [ ] 5.3 Scattering PDF: Lambertian `cos θ / π` (`scattering_pdf`), clamped to 0 below the
      surface
- [ ] Cornell box refactored to the book's version (`-w 7`; check against book listing
      `cornell-box`: light, walls, rotated boxes, 600×600, vfov 40, background black)

## 6. Playing with Importance Sampling
- [ ] 6.1 `Material::scattering_pdf(ray_in, hit, scattered)` added to the `Material` enum
      (Lambertian: `max(0, cos θ) / π`; others 0)
- [ ] 6.1 Ray color becomes `emitted + attenuation · scattering_pdf · color / pdf_value`
- [ ] 6.2 Uniform PDF `1/(2π)` over the hemisphere in the denominator (expect the same image,
      noisier)
- [ ] 6.3 Random hemispherical sampling (`random_on_hemisphere`) with PDF `1/(2π)`

## 7. Generating Random Directions
- [ ] 7.1 Random directions relative to the z axis (r₁, r₂ → φ, θ); `sphere_plot.cc` point
      plot (optional example)
- [ ] 7.2 Uniform hemisphere sampling; `cos_cubed.cc` integration check (example)
- [ ] 7.3 Cosine-weighted hemisphere sampling: `random_cosine_direction()` in
      `src/data/vector3.rs`; `cos_density.cc` integration check (example)
- [ ] Tests: `random_cosine_direction` has z ≥ 0, unit length, and mean z ≈ 2/3

## 8. Orthonormal Bases
- [ ] 8.1–8.2 Relative coordinates; building a basis from one vector (pick a non-parallel helper
      axis). We already have a private `plane_basis()` in `src/hittables/quad.rs` (section 6.7
      disks); consolidate it into the new ONB
- [ ] 8.3 `Onb` type (`src/data/onb.rs`): `u()`, `v()`, `w()`, `transform(vec)`. Could wrap
      `nalgebra::Rotation3` / `Matrix3`
- [ ] Lambertian `scatter` uses `Onb::from_w(normal).transform(random_cosine_direction())`
      with PDF `cos θ / π` (matches scattering PDF, so image = plain Lambertian, less noise)
- [ ] Isotropic: scattering PDF `1/(4π)`, uniform sphere PDF (`class-isotropic-impsample`)
- [ ] Tests: ONB orthonormal and right-handed; `transform(z) == w`

## 9. Sampling Lights Directly
- [ ] 9.1 PDF of a light: `distance² / (cos_light · area)` for a quad light
- [ ] 9.2 Light sampling in ray color: hard-coded Cornell light first (`ray-color-lights`),
      book's 10 spp comparison
- [ ] 9.3 Unidirectional light: `DiffuseLight::emitted` returns black on back faces
      (`if !rec.front_face return 0`). Needs `emitted(ray_in, hit_record, u, v, p)`; our
      `Material::emitted` already takes `&HitRecord` (book-1 chapter 6 change)
- [ ] **⚠** Book-2 scenes rely on two-sided lights (e.g. `simple_light`'s quad, the sphere light);
      check each scene still lights correctly, or flip quad orientations

## 10. Mixture Densities
- [ ] 10.1 `Pdf` enum with `value(direction) -> f64` and `generate() -> direction`
      (`src/pdf.rs`)
- [ ] 10.1 `SpherePdf` (uniform `1/(4π)`), `CosinePdf(Onb)` (`max(0, cos θ)/π`); ray color using
      the cosine PDF (`ray-color-cos-pdf`), back to 1000 spp comparison
- [ ] 10.2 `HittablePdf { objects, origin }`; add `pdf_value(origin, direction)` and
      `random(origin)` to `Hittable` (dispatch arms for each variant; default `0.0` and
      `(1, 0, 0)` like the book)
- [ ] 10.2 `Quad::pdf_value` / `Quad::random` (`quad-pdf`): needs the quad's `area`. **⚠**
      `PlanarShape` (book 2, 6.7) adds triangles/ellipses/annuli: area and uniform sampling per
      shape (triangle: area/2 with fold-over sampling; ellipse: π|u×v|; annulus: π|u×v|(1 − inner²)),
      or limit light sampling to parallelograms at first
- [ ] 10.3 `MixturePdf` (50/50 of light and cosine), ray color using it (`ray-color-mixture`)
- [ ] Add `World::lights` (Cornell light quad) and thread through rendering
- [ ] Tests: each PDF integrates to ≈ 1 over the sphere (Monte Carlo check); `generate()`
      samples are consistent with `value()` (histogram or moment check); quad
      `pdf_value` matches `dist²/(cos·area)`

## 11. Some Architectural Decisions
- [ ] Read-through only: book's reasoning about where the PDF lives (material vs. scatter
      record), specular handling, and the lights list. Record the Rust-specific choices in a doc
      comment on `Pdf` / `ScatterRecord`

## 12. Cleaning Up PDF Management
- [ ] 12.1 Diffuse vs. specular: `ScatterRecord { attenuation, pdf, skip_pdf, skip_pdf_ray }`;
      Lambertian → cosine PDF; Isotropic → sphere PDF
- [ ] 12.2 Specular: `Metal` and `Dielectric` set `skip_pdf = true` and return the explicit ray
      (`material-scatter`); ray color follows `skip_pdf_ray` directly (`ray-color-implicit`)
- [ ] 12.2 Cornell box with an aluminum box (`scene-cornell-al`: metal (0.8, 0.85, 0.88),
      fuzz 0); add as a world
- [ ] 12.3–12.4 Sampling a sphere object: `Sphere::pdf_value` (solid angle
      `1/(2π(1 − cos θ_max))`) and `Sphere::random` (`random_to_sphere` in a local ONB)
      (`sphere-pdf`); "sampling just the sphere" scene with a glass sphere
- [ ] 12.5 PDFs for hittable lists: `HittableVec::pdf_value` = average over objects,
      `HittableVec::random` = pick a random object (`density-mixture`). Also decide `Bvh`,
      `Translation`, `Rotation` (transform origin/direction into object space)
- [ ] 12.5 Final scene: Cornell box with the light quad **and** the glass sphere in `lights`
      (`scene-density-mixture`)
- [ ] 12.6 Surface acne: NaN-tolerant color output. **⚠** We *sum* samples per pixel
      (`render_frame`), so one NaN sample turns the whole pixel black. Zero NaN components per
      sample *before* accumulation (book does it in `write_color` on the averaged color), and
      add a test
- [ ] Tests: sphere `pdf_value` integrates to 1 over directions that hit; `random()` directions
      always hit the sphere; specular materials bypass the PDF path

## 13. The Rest of Your Life
- [ ] Read-through: pointers to further topics (spectral rendering, BSDFs, MIS, …). Optional
      follow-ups to list here: multiple importance sampling with balance heuristic,
      environment-map lights, BVH-aware light sampling

## Cross-cutting
- [ ] Before/after renders of the Cornell box at equal spp (book 2 brute force vs. light
      sampling vs. mixture) to show the variance drop
- [ ] Interactive mode (`src/util/interactive.rs`): keep working with the new `lights` argument
- [ ] Re-render book-1/book-2 scenes after the material changes (cosine-sampled Lambertian
      should not change their mean appearance, only the noise)
