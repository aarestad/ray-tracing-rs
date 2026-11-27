use crate::data::color64::{BLACK, Color64};
use crate::data::ray::Ray;
use crate::hittables::HitRecord;
use nalgebra::Vector3;

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
    /// Debug material (book 1, section 6.1): emits `0.5 * (normal + 1)` and never scatters,
    /// so each surface shows its normal as a color.
    NormalShaded,
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        match self {
            Material::Lambertian(m) => m.scatter(ray_in, hit_record),
            Material::Dielectric(m) => m.scatter(ray_in, hit_record),
            Material::DiffuseLight(m) => m.scatter(ray_in, hit_record),
            Material::Isotropic(m) => m.scatter(ray_in, hit_record),
            Material::Metal(m) => m.scatter(ray_in, hit_record),
            Material::NormalShaded => None,
        }
    }

    pub fn emitted(&self, hit_record: &HitRecord) -> Color64 {
        match self {
            Material::DiffuseLight(m) => {
                m.emitted(hit_record.u, hit_record.v, &hit_record.location)
            }
            Material::NormalShaded => {
                Color64(0.5 * (hit_record.normal.0 + Vector3::new(1., 1., 1.)))
            }
            _ => BLACK,
        }
    }
}
