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
        self.hit_interval(ray, t_min, t_max).is_some()
    }

    /// Ray segment `[t_min, t_max]` clipped to this box. Returns `[t_enter, t_exit]` along the ray.
    pub fn hit_interval(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(f64, f64)> {
        let mut t0 = t_min;
        let mut t1 = t_max;
        for idx in 0..3 {
            let inv_direction = 1. / ray.direction.0[idx];
            let mut ta = (self.minimum.0[idx] - ray.origin.0[idx]) * inv_direction;
            let mut tb = (self.maximum.0[idx] - ray.origin.0[idx]) * inv_direction;

            if inv_direction < 0. {
                mem::swap(&mut ta, &mut tb);
            }

            t0 = t0.max(ta);
            t1 = t1.min(tb);
            if t0 >= t1 {
                return None;
            }
        }
        Some((t0, t1))
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
