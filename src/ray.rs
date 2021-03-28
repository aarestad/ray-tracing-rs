use crate::point64::Point64;

pub struct Ray<'a> {
    pub origin: &'a Point64,
    pub direction: &'a Point64,
}

impl Ray<'_> {
    pub fn _point_at_parameter(&self, t: f64) -> Point64 {
        Point64(**self.origin + t * **self.direction)
    }
}