use crate::data::interval::Interval;
use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::HitRecord;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::materials::Material;
use nalgebra::Vector3;

/// Which region of the plane, in the planar coordinates (α, β) relative to Q along u and v,
/// counts as "inside" (book 2, section 6.7: additional 2D primitives).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlanarShape {
    /// Q is a corner; vertices Q, Q+u, Q+v, Q+u+v. Inside: 0 ≤ α, β ≤ 1.
    Parallelogram,
    /// Q is a corner; vertices Q, Q+u, Q+v. Inside: α, β ≥ 0 and α + β ≤ 1.
    Triangle,
    /// Q is the center; u and v are the semi-axes. Inside: α² + β² ≤ 1.
    Ellipse,
    /// Q is the center; u and v are the outer semi-axes, `inner` ∈ [0, 1) is the hole's size
    /// as a fraction of the outer one. Inside: inner ≤ √(α² + β²) ≤ 1.
    Annulus { inner: f64 },
}

impl PlanarShape {
    /// Texture coordinates for a point at planar coordinates (α, β), or `None` if it lies
    /// outside the shape. Centered shapes map their [-1, 1] extent onto [0, 1].
    fn interior_uv(&self, alpha: f64, beta: f64) -> Option<(f64, f64)> {
        let unit = 0.0..=1.0;
        let centered_uv = (0.5 * (alpha + 1.), 0.5 * (beta + 1.));
        match *self {
            PlanarShape::Parallelogram => {
                (unit.contains(&alpha) && unit.contains(&beta)).then_some((alpha, beta))
            }
            PlanarShape::Triangle => {
                (alpha >= 0. && beta >= 0. && alpha + beta <= 1.).then_some((alpha, beta))
            }
            PlanarShape::Ellipse => (alpha * alpha + beta * beta <= 1.).then_some(centered_uv),
            PlanarShape::Annulus { inner } => {
                let r = (alpha * alpha + beta * beta).sqrt();
                (inner <= r && r <= 1.).then_some(centered_uv)
            }
        }
    }
}

/// A planar primitive: a region of the plane through Q spanned by edge vectors u and v.
/// By default a parallelogram (the book's `quad`); see [`PlanarShape`] for the other shapes.
#[derive(Clone)]
pub struct Quad {
    pub q: Point64,
    pub u: Point64,
    pub v: Point64,
    pub material: Material,
    shape: PlanarShape,
    // Cached derived values
    w: Point64,      // n / (n·n), used to compute planar coordinates
    normal: Point64, // unit normal of the containing plane
    plane_d: f64,    // plane equation constant: normal · P = plane_d
}

impl Quad {
    /// Parallelogram with corner `q` and edges `u`, `v`.
    pub fn new(q: Point64, u: Point64, v: Point64, material: Material) -> Self {
        Self::with_shape(q, u, v, PlanarShape::Parallelogram, material)
    }

    /// Triangle with vertices `q`, `q + u`, `q + v`.
    pub fn triangle(q: Point64, u: Point64, v: Point64, material: Material) -> Self {
        Self::with_shape(q, u, v, PlanarShape::Triangle, material)
    }

    /// Ellipse centered at `center` with semi-axis vectors `u` and `v`. (Non-perpendicular
    /// axes give a sheared ellipse, which is still an ellipse.)
    pub fn ellipse(center: Point64, u: Point64, v: Point64, material: Material) -> Self {
        Self::with_shape(center, u, v, PlanarShape::Ellipse, material)
    }

    /// Circular disk of `radius` centered at `center`, facing along `normal`.
    pub fn disk(center: Point64, normal: Vector3<f64>, radius: f64, material: Material) -> Self {
        let (u, v) = plane_basis(&normal);
        Self::ellipse(center, Point64(u * radius), Point64(v * radius), material)
    }

    /// Ring centered at `center` with outer semi-axes `u`, `v`; the hole is `inner` (in [0, 1))
    /// times the outer size.
    pub fn annulus(
        center: Point64,
        u: Point64,
        v: Point64,
        inner: f64,
        material: Material,
    ) -> Self {
        assert!(
            (0.0..1.0).contains(&inner),
            "annulus inner fraction must be in [0, 1), got {inner}"
        );
        Self::with_shape(center, u, v, PlanarShape::Annulus { inner }, material)
    }

    pub fn with_shape(
        q: Point64,
        u: Point64,
        v: Point64,
        shape: PlanarShape,
        material: Material,
    ) -> Self {
        let n = u.0.cross(&v.0);
        let normal = Point64(n.normalize());
        let plane_d = normal.0.dot(&q.0);
        let w = Point64(n / n.dot(&n));
        Self {
            q,
            u,
            v,
            material,
            shape,
            w,
            normal,
            plane_d,
        }
    }

    pub fn bounding_box(&self, _: f64, _: f64) -> Option<AxisAlignedBoundingBox> {
        let (q, u, v) = (self.q, self.u, self.v);
        let bbox = match self.shape {
            PlanarShape::Parallelogram => AxisAlignedBoundingBox::from_corners(q, q + u + v)
                .surrounding_box_with(&AxisAlignedBoundingBox::from_corners(q + u, q + v)),
            PlanarShape::Triangle => AxisAlignedBoundingBox::from_corners(q, q + u)
                .surrounding_box_with(&AxisAlignedBoundingBox::from_corners(q, q + v)),
            // The ellipse sits inside the parallelogram with corners Q ± u ± v.
            PlanarShape::Ellipse | PlanarShape::Annulus { .. } => {
                AxisAlignedBoundingBox::from_corners(q - u - v, q + u + v).surrounding_box_with(
                    &AxisAlignedBoundingBox::from_corners(q + u - v, q - u + v),
                )
            }
        };
        Some(bbox.padded())
    }

    pub fn is_hit_by(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = self.normal.0.dot(&ray.direction.0);
        // Ray is parallel to the plane — no hit.
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.plane_d - self.normal.0.dot(&ray.origin.0)) / denom;
        if !ray_t.surrounds(t) {
            return None;
        }

        // Check whether the intersection lies within the shape using planar coordinates.
        let p = ray.point_at_parameter(t) - self.q;
        let alpha = self.w.0.dot(&p.0.cross(&self.v.0));
        let beta = self.w.0.dot(&self.u.0.cross(&p.0));
        let uv = self.shape.interior_uv(alpha, beta)?;

        Some(HitRecord::new(
            t,
            ray,
            self.normal,
            self.material.clone(),
            uv,
        ))
    }
}

/// Two unit vectors perpendicular to `normal` and to each other, oriented so that
/// u × v points along `normal`.
fn plane_basis(normal: &Vector3<f64>) -> (Vector3<f64>, Vector3<f64>) {
    let n = normal.normalize();
    // Pick the world axis least aligned with n to avoid a degenerate cross product.
    let helper = if n.x.abs() < 0.9 {
        Vector3::x()
    } else {
        Vector3::y()
    };
    let v = n.cross(&helper).normalize();
    let u = v.cross(&n);
    (u, v)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::materials::dielectric::Dielectric;
    use approx::assert_abs_diff_eq;

    /// Quad in the z = 0 plane: corner (1, 2, 0), edges u = (2, 0, 0), v = (0, 4, 0).
    fn quad() -> Quad {
        Quad::new(
            Point64::new(1., 2., 0.),
            Point64::new(2., 0., 0.),
            Point64::new(0., 4., 0.),
            Material::Dielectric(Dielectric {
                index_of_refraction: 1.5,
            }),
        )
    }

    fn ray_down_z(x: f64, y: f64, z0: f64, dz: f64) -> Ray {
        Ray {
            origin: Point64::new(x, y, z0),
            direction: Point64::new(0., 0., dz),
            exposure_time: 0.,
        }
    }

    const RANGE: Interval = Interval::new(0.001, f64::INFINITY);

    #[test]
    fn hit_inside_gives_planar_uv() {
        // (2.5, 5) is 3/4 along u and 3/4 along v.
        let hr = quad()
            .is_hit_by(&ray_down_z(2.5, 5., 3., -1.), RANGE)
            .unwrap();
        assert_abs_diff_eq!(hr.value, 3., epsilon = 1e-12);
        assert_abs_diff_eq!(hr.u, 0.75, epsilon = 1e-12);
        assert_abs_diff_eq!(hr.v, 0.75, epsilon = 1e-12);
        assert_abs_diff_eq!(hr.location.0, Point64::new(2.5, 5., 0.).0, epsilon = 1e-12);
        // u × v points +z, toward the ray origin: front face.
        assert!(hr.front_face);
        assert_abs_diff_eq!(hr.normal.0, Point64::new(0., 0., 1.).0, epsilon = 1e-12);
    }

    #[test]
    fn hit_from_behind_is_back_face() {
        let hr = quad()
            .is_hit_by(&ray_down_z(2., 4., -3., 1.), RANGE)
            .unwrap();
        assert!(!hr.front_face);
        assert_abs_diff_eq!(hr.normal.0, Point64::new(0., 0., -1.).0, epsilon = 1e-12);
    }

    #[test]
    fn corners_hit_and_outside_misses() {
        let q = quad();
        for (x, y) in [(1., 2.), (3., 2.), (1., 6.), (3., 6.)] {
            assert!(
                q.is_hit_by(&ray_down_z(x, y, 3., -1.), RANGE).is_some(),
                "corner ({x},{y})"
            );
        }
        for (x, y) in [(0.9, 4.), (3.1, 4.), (2., 1.9), (2., 6.1)] {
            assert!(
                q.is_hit_by(&ray_down_z(x, y, 3., -1.), RANGE).is_none(),
                "outside ({x},{y})"
            );
        }
    }

    #[test]
    fn parallel_ray_and_t_range() {
        let q = quad();
        let parallel = Ray {
            origin: Point64::new(0., 4., 0.),
            direction: Point64::new(1., 0., 0.),
            exposure_time: 0.,
        };
        assert!(q.is_hit_by(&parallel, RANGE).is_none());
        assert!(
            q.is_hit_by(&ray_down_z(2., 4., 3., -1.), Interval::new(0.001, 2.))
                .is_none()
        );
        // Plane is behind the ray.
        assert!(q.is_hit_by(&ray_down_z(2., 4., 3., 1.), RANGE).is_none());
    }

    #[test]
    fn non_rectangular_parallelogram() {
        // Sheared: v leans in +x. (2.5, 1.9) is inside (α = 0.6, β = 0.95); (0.2, 1.9) is
        // inside the bounding rectangle but left of the slanted edge (α < 0).
        let q = Quad::new(
            Point64::new(0., 0., 0.),
            Point64::new(1., 0., 0.),
            Point64::new(2., 2., 0.),
            Material::Dielectric(Dielectric {
                index_of_refraction: 1.5,
            }),
        );
        assert!(q.is_hit_by(&ray_down_z(2.5, 1.9, 1., -1.), RANGE).is_some());
        assert!(q.is_hit_by(&ray_down_z(0.2, 1.9, 1., -1.), RANGE).is_none());
    }

    fn glass() -> Material {
        Material::Dielectric(Dielectric {
            index_of_refraction: 1.5,
        })
    }

    fn hits(shape: &Quad, x: f64, y: f64) -> bool {
        shape.is_hit_by(&ray_down_z(x, y, 3., -1.), RANGE).is_some()
    }

    #[test]
    fn triangle_interior() {
        // Vertices (0,0), (2,0), (0,2) in the z = 0 plane.
        let t = Quad::triangle(
            Point64::new(0., 0., 0.),
            Point64::new(2., 0., 0.),
            Point64::new(0., 2., 0.),
            glass(),
        );
        assert!(hits(&t, 0.5, 0.5));
        assert!(hits(&t, 0.99, 0.99)); // just inside the hypotenuse
        assert!(!hits(&t, 1.01, 1.01)); // just outside it (inside the parallelogram)
        assert!(!hits(&t, -0.1, 0.5));
        let hr = t.is_hit_by(&ray_down_z(0.5, 1.0, 3., -1.), RANGE).unwrap();
        assert_abs_diff_eq!(hr.u, 0.25, epsilon = 1e-12);
        assert_abs_diff_eq!(hr.v, 0.5, epsilon = 1e-12);
    }

    #[test]
    fn ellipse_interior_and_uv() {
        // Center (1, 1), semi-axes 2 along x and 1 along y.
        let e = Quad::ellipse(
            Point64::new(1., 1., 0.),
            Point64::new(2., 0., 0.),
            Point64::new(0., 1., 0.),
            glass(),
        );
        assert!(hits(&e, 1., 1.));
        assert!(hits(&e, 2.9, 1.)); // near the end of the long axis
        assert!(!hits(&e, 3.1, 1.));
        assert!(hits(&e, 1., 1.9));
        assert!(!hits(&e, 1., 2.1));
        // Inside the bounding rectangle but outside the ellipse (α = β = 0.8).
        assert!(!hits(&e, 2.6, 1.8));
        let hr = e.is_hit_by(&ray_down_z(1., 1., 3., -1.), RANGE).unwrap();
        assert_abs_diff_eq!(hr.u, 0.5, epsilon = 1e-12);
        assert_abs_diff_eq!(hr.v, 0.5, epsilon = 1e-12);
    }

    #[test]
    fn disk_is_circular_and_faces_its_normal() {
        let normal = Vector3::new(1., 2., 3.).normalize();
        let center = Point64::new(4., -1., 2.);
        let d = Quad::disk(center, normal, 1.5, glass());
        assert_abs_diff_eq!(d.normal.0, normal, epsilon = 1e-12);
        let (a, b) = plane_basis(&normal);
        for angle in (0..12).map(|k| k as f64 * std::f64::consts::TAU / 12.) {
            let dir = a * angle.cos() + b * angle.sin();
            for (r, expect) in [(1.49, true), (1.51, false)] {
                let target = center.0 + dir * r;
                let ray = Ray {
                    origin: Point64(target + normal * 5.),
                    direction: Point64(-normal),
                    exposure_time: 0.,
                };
                assert_eq!(d.is_hit_by(&ray, RANGE).is_some(), expect, "r = {r}");
            }
        }
    }

    #[test]
    fn annulus_has_a_hole() {
        let a = Quad::annulus(
            Point64::new(0., 0., 0.),
            Point64::new(2., 0., 0.),
            Point64::new(0., 2., 0.),
            0.5,
            glass(),
        );
        assert!(!hits(&a, 0., 0.)); // center
        assert!(!hits(&a, 0.9, 0.)); // in the hole (r = 0.45)
        assert!(hits(&a, 1.1, 0.)); // on the ring (r = 0.55)
        assert!(hits(&a, 0., -1.9));
        assert!(!hits(&a, 0., -2.1)); // outside
    }

    #[test]
    fn shaped_bounding_boxes() {
        let t = Quad::triangle(
            Point64::new(0., 0., 0.),
            Point64::new(2., 0., 0.),
            Point64::new(0., 3., 0.),
            glass(),
        )
        .bounding_box(0., 1.)
        .unwrap();
        assert_abs_diff_eq!(t.minimum.x(), 0.);
        assert_abs_diff_eq!(t.maximum.x(), 2.);
        assert_abs_diff_eq!(t.maximum.y(), 3.);
        let e = Quad::ellipse(
            Point64::new(1., 1., 1.),
            Point64::new(2., 0., 0.),
            Point64::new(0., 0., 1.),
            glass(),
        )
        .bounding_box(0., 1.)
        .unwrap();
        assert_abs_diff_eq!(e.minimum.x(), -1.);
        assert_abs_diff_eq!(e.maximum.x(), 3.);
        assert_abs_diff_eq!(e.minimum.z(), 0.);
        assert_abs_diff_eq!(e.maximum.z(), 2.);
        assert!(e.maximum.y() - e.minimum.y() > 0.); // flat axis is padded
    }

    #[test]
    #[should_panic(expected = "inner fraction")]
    fn annulus_rejects_bad_inner_fraction() {
        let _ = Quad::annulus(
            Point64::new(0., 0., 0.),
            Point64::new(1., 0., 0.),
            Point64::new(0., 1., 0.),
            1.0,
            glass(),
        );
    }
}
