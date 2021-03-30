use crate::color64::Color64;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub struct ScatterRecord {
    pub hit_record: HitRecord,
    pub attenuation: Color64,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
}
