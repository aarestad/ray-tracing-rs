use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::{HitRecord, Hittable};
use crate::materials::Material;
use std::sync::Arc;

pub enum AxisAlignment {
    X,
    Y,
    Z,
}

impl AxisAlignment {
    pub const fn to_usize(&self) -> usize {
        match self {
            AxisAlignment::X => 0,
            AxisAlignment::Y => 1,
            AxisAlignment::Z => 2,
        }
    }
}

/// min and max are a 2-dimensional pair depending on axis alignment
/// X -> (y, z)
/// Y -> (x, z)
/// Z -> (x, y)
pub struct AxisAlignedRect {
    pub material: Arc<dyn Material>,
    pub min: (f64, f64),
    pub max: (f64, f64),
    pub axis_value: f64,
    pub axis_alignment: AxisAlignment,
}

impl Hittable for AxisAlignedRect {
    fn bounding_box(&self, _: f64, _: f64) -> Option<AxisAlignedBoundingBox> {
        match self.axis_alignment {
            AxisAlignment::X => Some(AxisAlignedBoundingBox {
                minimum: Point64::new(self.axis_value - 0.0001, self.min.0, self.min.1),
                maximum: Point64::new(self.axis_value + 0.0001, self.max.0, self.max.1),
            }),
            AxisAlignment::Y => Some(AxisAlignedBoundingBox {
                minimum: Point64::new(self.min.0, self.axis_value - 0.0001, self.min.1),
                maximum: Point64::new(self.max.0, self.axis_value + 0.0001, self.max.1),
            }),
            AxisAlignment::Z => Some(AxisAlignedBoundingBox {
                minimum: Point64::new(self.min.0, self.min.1, self.axis_value - 0.0001),
                maximum: Point64::new(self.max.0, self.max.1, self.axis_value + 0.0001),
            }),
        }
    }

    fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord> {
        let t = (self.axis_value - ray.origin.0[self.axis_alignment.to_usize()])
            / ray.direction.0[self.axis_alignment.to_usize()];

        if !(min_value..max_value).contains(&t) {
            return None;
        }

        let (c1, c2): (f64, f64) = match self.axis_alignment {
            AxisAlignment::X => (
                ray.origin.y() + t * ray.direction.y(),
                ray.origin.z() + t * ray.direction.z(),
            ),
            AxisAlignment::Y => (
                ray.origin.x() + t * ray.direction.x(),
                ray.origin.z() + t * ray.direction.z(),
            ),
            AxisAlignment::Z => (
                ray.origin.x() + t * ray.direction.x(),
                ray.origin.y() + t * ray.direction.y(),
            ),
        };

        if !(self.min.0..self.max.0).contains(&c1) || !(self.min.1..self.max.1).contains(&c2) {
            return None;
        }

        Some(HitRecord::new(
            t,
            ray,
            Point64::new(0., 0., 1.),
            self.material.clone(),
            (
                (c1 - self.min.0) / (self.max.0 - self.min.0),
                (c2 - self.min.1) / (self.max.1 - self.min.1),
            ),
        ))
    }
}
