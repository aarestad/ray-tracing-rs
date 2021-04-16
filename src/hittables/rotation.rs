use crate::hittables::axis_aligned_rect::AxisAlignment;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::{Hittable, HitRecord};
use crate::data::ray::Ray;

pub struct Rotation {
    hittable: Box<dyn Hittable>,
    axis_alignment: AxisAlignment,
    sin_cos_theta: (f64, f64),
    bounding_box: Optional<AxisAlignedBoundingBox>,
}

impl Rotation {
    pub fn new(hittable: Box<dyn Hittable>, theta: f64) -> Rotation {

    }
}

impl Hittable for Rotation {
    fn bounding_box(&self, _: f64, _: f64) -> Option<AxisAlignedBoundingBox> {
        self.bounding_box
    }

    fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord> {
        todo!()
    }
}