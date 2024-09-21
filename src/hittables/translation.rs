use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::{HitRecord, Hittable};
use nalgebra::Vector3;
use std::sync::Arc;

pub struct Translation {
    pub hittable: Arc<dyn Hittable>,
    pub offset: Vector3<f64>,
}

impl Hittable for Translation {
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AxisAlignedBoundingBox> {
        let orig_bounding_box = self.hittable.bounding_box(time0, time1);

        match orig_bounding_box {
            None => orig_bounding_box,

            Some(aabb) => Some(AxisAlignedBoundingBox {
                minimum: Point64(aabb.minimum.0 + self.offset),
                maximum: Point64(aabb.maximum.0 + self.offset),
            }),
        }
    }

    fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord> {
        let moved_ray = Ray {
            origin: Point64(ray.origin.0 - self.offset),
            direction: ray.direction,
            exposure_time: ray.exposure_time,
        };

        let opt_hit_record = self.hittable.is_hit_by(&moved_ray, min_value, max_value);

        match opt_hit_record {
            Some(hr) => Some(HitRecord::new(
                hr.value,
                &moved_ray,
                hr.normal,
                hr.material.clone(),
                (hr.u, hr.v),
            )),
            None => None,
        }
    }
}
