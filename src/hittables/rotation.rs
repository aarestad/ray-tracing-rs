#![allow(dead_code)]

use crate::data::ray::Ray;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::axis_aligned_rect::AxisAlignment;
use crate::hittables::{HitRecord, Hittable};

pub struct Rotation {
    hittable: Box<dyn Hittable>,
    axis_alignment: AxisAlignment,
    sin_cos_theta: (f64, f64),
    bounding_box: Option<AxisAlignedBoundingBox>,
}

impl Rotation {
    pub fn new(_: Box<dyn Hittable>, _: f64) -> Rotation {
        todo!()
    }
}

impl Hittable for Rotation {
    fn bounding_box(&self, _: f64, _: f64) -> Option<AxisAlignedBoundingBox> {
        self.bounding_box
    }

    fn is_hit_by(&self, _: &Ray, _: f64, _: f64) -> Option<HitRecord> {
        todo!()
    }
}
