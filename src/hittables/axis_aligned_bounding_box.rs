use crate::data::point64::Point64;
use crate::data::ray::Ray;
use std::mem;

#[derive(PartialEq, Copy, Clone)]
pub struct AxisAlignedBoundingBox {
    pub minimum: Point64,
    pub maximum: Point64,
}

impl AxisAlignedBoundingBox {
    pub fn is_hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        (0..3).all(|idx| {
            let inv_direction = 1. / ray.direction.0[idx];
            let mut t0 = (self.minimum.0[idx] - ray.origin.0[idx]) * inv_direction;
            let mut t1 = (self.maximum.0[idx] - ray.origin.0[idx]) * inv_direction;

            if inv_direction < 0. {
                mem::swap(&mut t0, &mut t1);
            }

            t0.max(t_min) < t1.min(t_max)
        })
    }

    pub fn surrounding_box_with(self, other: &AxisAlignedBoundingBox) -> AxisAlignedBoundingBox {
        if self == *other {
            return self;
        };

        let minimum = Point64::new(
            self.minimum.x().min(other.minimum.x()),
            self.minimum.y().min(other.minimum.y()),
            self.minimum.z().min(other.minimum.z()),
        );

        let maximum = Point64::new(
            self.maximum.x().max(other.maximum.x()),
            self.maximum.y().max(other.maximum.y()),
            self.maximum.z().max(other.maximum.z()),
        );

        AxisAlignedBoundingBox { minimum, maximum }
    }
}
