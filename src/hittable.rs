use crate::point64::Point64;
use crate::ray::Ray;

pub struct HitRecord {
    pub value: f64,
    pub location: Point64,
    pub normal: Point64,
}

pub trait Hittable {
    fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord>;
}
