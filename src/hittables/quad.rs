use crate::data::interval::Interval;
use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::HitRecord;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::materials::Material;

/// A parallelogram defined by a corner Q and two edge vectors u and v.
/// The four vertices are Q, Q+u, Q+v, and Q+u+v.
#[derive(Clone)]
pub struct Quad {
    pub q: Point64,
    pub u: Point64,
    pub v: Point64,
    pub material: Material,
    // Cached derived values
    w: Point64,      // n / (n·n), used to compute planar coordinates
    normal: Point64, // unit normal of the containing plane
    plane_d: f64,    // plane equation constant: normal · P = plane_d
}

impl Quad {
    pub fn new(q: Point64, u: Point64, v: Point64, material: Material) -> Self {
        let n = u.0.cross(&v.0);
        let normal = Point64(n.normalize());
        let plane_d = normal.0.dot(&q.0);
        let w = Point64(n / n.dot(&n));
        Self {
            q,
            u,
            v,
            material,
            w,
            normal,
            plane_d,
        }
    }

    pub fn bounding_box(&self, _: f64, _: f64) -> Option<AxisAlignedBoundingBox> {
        let diag1 = AxisAlignedBoundingBox::from_corners(self.q, self.q + self.u + self.v);
        let diag2 = AxisAlignedBoundingBox::from_corners(self.q + self.u, self.q + self.v);
        Some(diag1.surrounding_box_with(&diag2).padded())
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

        // Check whether the intersection lies within the quad using planar coordinates.
        let p = ray.point_at_parameter(t) - self.q;
        let alpha = self.w.0.dot(&p.0.cross(&self.v.0));
        let beta = self.w.0.dot(&self.u.0.cross(&p.0));

        if !(0.0..=1.0).contains(&alpha) || !(0.0..=1.0).contains(&beta) {
            return None;
        }

        Some(HitRecord::new(
            t,
            ray,
            self.normal,
            self.material.clone(),
            (alpha, beta),
        ))
    }
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
}
