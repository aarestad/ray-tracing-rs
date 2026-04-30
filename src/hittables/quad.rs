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

fn aabb_from_corners(a: Point64, b: Point64) -> AxisAlignedBoundingBox {
    AxisAlignedBoundingBox {
        minimum: Point64::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z())),
        maximum: Point64::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z())),
    }
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
        let diag1 = aabb_from_corners(self.q, self.q + self.u + self.v);
        let diag2 = aabb_from_corners(self.q + self.u, self.q + self.v);
        let mut bbox = diag1.surrounding_box_with(&diag2);
        // Pad any axis-aligned thin dimension to avoid degenerate AABBs.
        const DELTA: f64 = 0.0001;
        for i in 0..3 {
            if bbox.maximum.0[i] - bbox.minimum.0[i] < DELTA {
                bbox.minimum.0[i] -= DELTA;
                bbox.maximum.0[i] += DELTA;
            }
        }
        Some(bbox)
    }

    pub fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord> {
        let denom = self.normal.0.dot(&ray.direction.0);
        // Ray is parallel to the plane — no hit.
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.plane_d - self.normal.0.dot(&ray.origin.0)) / denom;
        if !(min_value..max_value).contains(&t) {
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
