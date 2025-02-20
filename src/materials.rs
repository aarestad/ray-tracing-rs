use crate::data::color64::{BLACK, Color64};
use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::HitRecord;

pub mod dielectric;
pub mod diffuse_light;
pub mod lambertian;
pub mod metal;

pub struct ScatterRecord {
    #[allow(dead_code)]
    pub hit_record: HitRecord,
    pub attenuation: Color64,
    pub scattered: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;

    fn emitted(&self, _u: f64, _v: f64, _point: &Point64) -> Color64 {
        BLACK
    }
}
