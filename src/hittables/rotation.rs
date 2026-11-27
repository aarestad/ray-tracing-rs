use crate::data::interval::Interval;
use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::{HitRecord, Hittable};
use nalgebra::{Rotation3, Vector3};

#[derive(Clone)]
pub enum AxisAlignment {
    X,
    Y,
    Z,
}

/// Rotates child geometry about an axis through the origin (right-handed, angle in radians).
#[derive(Clone)]
pub struct Rotation {
    hittable: Box<Hittable>,
    rot: Rotation3<f64>,
    inv_rot: Rotation3<f64>,
    bounding_box: Option<AxisAlignedBoundingBox>,
}

impl Rotation {
    pub fn new(
        hittable: Box<Hittable>,
        axis_alignment: AxisAlignment,
        angle_radians: f64,
        time0: f64,
        time1: f64,
    ) -> Self {
        let axis = match axis_alignment {
            AxisAlignment::X => Vector3::x_axis(),
            AxisAlignment::Y => Vector3::y_axis(),
            AxisAlignment::Z => Vector3::z_axis(),
        };
        let rot = Rotation3::from_axis_angle(&axis, angle_radians);
        let inv_rot = rot.inverse();
        let bounding_box = hittable
            .bounding_box(time0, time1)
            .map(|bb| rotate_aabb(&bb, &rot));

        Self {
            hittable,
            rot,
            inv_rot,
            bounding_box,
        }
    }

    pub fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AxisAlignedBoundingBox> {
        self.bounding_box
    }

    pub fn is_hit_by(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let origin = Point64(self.inv_rot * ray.origin.0);
        let direction = Point64(self.inv_rot * ray.direction.0);
        let local_ray = Ray {
            origin,
            direction,
            exposure_time: ray.exposure_time,
        };

        // Book 2, section 8.2: hit in object space, then rotate the hit point and normal back.
        // The child's normal already faces against the local ray, so the rotated normal faces
        // against the world ray; keep the child's front_face rather than re-deriving it.
        let hr = self.hittable.is_hit_by(&local_ray, ray_t)?;
        Some(HitRecord {
            location: Point64(self.rot * hr.location.0),
            normal: Point64(self.rot * hr.normal.0),
            ..hr
        })
    }
}

fn rotate_aabb(bb: &AxisAlignedBoundingBox, rot: &Rotation3<f64>) -> AxisAlignedBoundingBox {
    let mi = bb.minimum.0;
    let ma = bb.maximum.0;
    let mut out_min = Vector3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    let mut out_max = Vector3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

    for i in 0..8_u8 {
        let p = Vector3::new(
            if i & 1 == 0 { mi.x } else { ma.x },
            if i & 2 == 0 { mi.y } else { ma.y },
            if i & 4 == 0 { mi.z } else { ma.z },
        );
        let q = rot * p;
        out_min.x = out_min.x.min(q.x);
        out_min.y = out_min.y.min(q.y);
        out_min.z = out_min.z.min(q.z);
        out_max.x = out_max.x.max(q.x);
        out_max.y = out_max.y.max(q.y);
        out_max.z = out_max.z.max(q.z);
    }

    AxisAlignedBoundingBox {
        minimum: Point64(out_min),
        maximum: Point64(out_max),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::hittables::cuboid::Cuboid;
    use crate::hittables::sphere::Sphere;
    use crate::materials::Material;
    use crate::materials::dielectric::Dielectric;
    use approx::assert_abs_diff_eq;
    use std::f64::consts::FRAC_PI_2;

    fn glass() -> Material {
        Material::Dielectric(Dielectric {
            index_of_refraction: 1.5,
        })
    }

    const RANGE: Interval = Interval::new(0.001, f64::INFINITY);

    /// A 2×1×1 box at the origin, rotated 90° about y, so its long side runs along z.
    fn rotated_box() -> Rotation {
        Rotation::new(
            Box::new(Hittable::Cuboid(Cuboid::new(
                Point64::new(0., 0., 0.),
                Point64::new(2., 1., 1.),
                glass(),
            ))),
            AxisAlignment::Y,
            FRAC_PI_2,
            0.,
            1.,
        )
    }

    #[test]
    fn rotated_box_hit_location_and_normal() {
        // After rotating +90° about y, (x, y, z) -> (z, y, -x): the box spans
        // x ∈ [0, 1], z ∈ [-2, 0].
        let r = Ray {
            origin: Point64::new(5., 0.5, -1.5),
            direction: Point64::new(-1., 0., 0.),
            exposure_time: 0.,
        };
        let hr = rotated_box().is_hit_by(&r, RANGE).unwrap();
        assert_abs_diff_eq!(hr.value, 4., epsilon = 1e-9);
        assert_abs_diff_eq!(hr.location.0, Point64::new(1., 0.5, -1.5).0, epsilon = 1e-9);
        assert_abs_diff_eq!(hr.normal.0, Point64::new(1., 0., 0.).0, epsilon = 1e-9);
        assert!(hr.front_face);
    }

    #[test]
    fn rotated_bounding_box() {
        let b = rotated_box().bounding_box(0., 1.).unwrap();
        assert_abs_diff_eq!(b.minimum.0, Point64::new(0., 0., -2.).0, epsilon = 1e-9);
        assert_abs_diff_eq!(b.maximum.0, Point64::new(1., 1., 0.).0, epsilon = 1e-9);
    }

    #[test]
    fn hit_from_inside_rotated_sphere_is_a_back_face() {
        let s = Rotation::new(
            Box::new(Hittable::Sphere(Sphere {
                center: Point64::new(3., 0., 0.),
                radius: 1.,
                material: glass(),
            })),
            AxisAlignment::Z,
            FRAC_PI_2,
            0.,
            1.,
        );
        // Rotating +90° about z moves the center (3, 0, 0) to (0, 3, 0).
        let r = Ray {
            origin: Point64::new(0., 3., 0.),
            direction: Point64::new(0., 0., 1.),
            exposure_time: 0.,
        };
        let hr = s.is_hit_by(&r, RANGE).unwrap();
        assert!(!hr.front_face);
        assert_abs_diff_eq!(hr.location.0, Point64::new(0., 3., 1.).0, epsilon = 1e-9);
        assert_abs_diff_eq!(hr.normal.0, Point64::new(0., 0., -1.).0, epsilon = 1e-9);
    }
}
