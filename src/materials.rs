use crate::data::color64::{BLACK, Color64};
use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::HitRecord;

pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

use dielectric::Dielectric;
use diffuse_light::DiffuseLight;
pub use isotropic::Isotropic;
use lambertian::Lambertian;
use metal::Metal;

pub struct ScatterRecord {
    #[allow(dead_code)]
    pub hit_record: HitRecord,
    pub attenuation: Color64,
    pub scattered: Ray,
}

#[derive(Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
    Isotropic(Isotropic),
    Metal(Metal),
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        match self {
            Material::Lambertian(m) => m.scatter(ray_in, hit_record),
            Material::Dielectric(m) => m.scatter(ray_in, hit_record),
            Material::DiffuseLight(m) => m.scatter(ray_in, hit_record),
            Material::Isotropic(m) => m.scatter(ray_in, hit_record),
            Material::Metal(m) => m.scatter(ray_in, hit_record),
        }
    }

    pub fn emitted(&self, u: f64, v: f64, point: &Point64) -> Color64 {
        match self {
            Material::DiffuseLight(m) => m.emitted(u, v, point),
            _ => BLACK,
        }
    }
}
