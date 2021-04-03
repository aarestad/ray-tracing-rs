use crate::data::point64::Point64;

pub struct Ray {
    pub origin: Point64,
    pub direction: Point64,
    pub exposure_time: f64,
}

impl Ray {
    pub fn point_at_parameter(&self, t: f64) -> Point64 {
        Point64(*self.origin + t * *self.direction)
    }
}