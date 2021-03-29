use crate::point64::Point64;
use crate::ray::Ray;

pub struct HitRecord {
    pub value: f64,
    pub location: Point64,
    pub normal: Point64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(value: f64, ray: &Ray, outward_normal: Point64) -> HitRecord {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;

        let normal = if front_face {
            outward_normal
        } else {
            Point64(-*outward_normal)
        };

        HitRecord {
            value,
            location: ray.point_at_parameter(value),
            normal,
            front_face,
        }
    }
}

pub trait Hittable {
    fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord>;
}
