use crate::data::color64::Color64;
use crate::data::ray::Ray;
use crate::hittables::HitRecord;

pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub struct ScatterRecord {
    pub hit_record: HitRecord,
    pub attenuation: Color64,
    pub scattered: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
}
