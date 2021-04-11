#![allow(clippy::upper_case_acronyms)] // it doesn't like "XYRect"

use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::{HitRecord, Hittable};
use crate::materials::Material;
use std::sync::Arc;

pub struct XYRect {
    pub material: Arc<dyn Material>,
    pub xy0: (f64, f64),
    pub xy1: (f64, f64),
    pub z_value: f64,
}

impl Hittable for XYRect {
    fn bounding_box(&self, _: f64, _: f64) -> Option<AxisAlignedBoundingBox> {
        Some(AxisAlignedBoundingBox {
            minimum: Point64::new(self.xy0.0, self.xy0.1, self.z_value - 0.0001),
            maximum: Point64::new(self.xy1.0, self.xy1.1, self.z_value + 0.0001),
        })
    }

    fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord> {
        let t = (self.z_value - ray.origin.z()) / ray.direction.z();

        if !(min_value..max_value).contains(&t) {
            return None;
        }

        let x = ray.origin.x() + t * ray.direction.x();
        let y = ray.origin.y() + t * ray.direction.y();

        if !(self.xy0.0..self.xy1.0).contains(&x) || !(self.xy0.1..self.xy1.1).contains(&y) {
            return None;
        }

        Some(HitRecord::new(
            t,
            ray,
            Point64::new(0., 0., 1.),
            self.material.clone(),
            (
                (x - self.xy0.0) / (self.xy1.0 - self.xy0.0),
                (y - self.xy0.1) / (self.xy1.1 - self.xy0.1),
            ),
        ))
    }
}
