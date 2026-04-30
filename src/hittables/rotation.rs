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

    pub fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord> {
        let origin = Point64(self.inv_rot * ray.origin.0);
        let direction = Point64(self.inv_rot * ray.direction.0);
        let local_ray = Ray {
            origin,
            direction,
            exposure_time: ray.exposure_time,
        };

        let hr = self.hittable.is_hit_by(&local_ray, min_value, max_value)?;
        let world_normal = (self.rot * hr.normal.0).normalize();

        Some(HitRecord::new(
            hr.value,
            ray,
            Point64(world_normal),
            hr.material,
            (hr.u, hr.v),
        ))
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
