use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::data::vector3::random_in_unit_sphere;
use crate::hittables::HitRecord;
use crate::materials::ScatterRecord;
use crate::textures::Texture;

#[derive(Clone)]
pub struct Isotropic {
    pub albedo: Texture,
}

impl Isotropic {
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            hit_record: hit_record.clone(),
            attenuation: self
                .albedo
                .value(hit_record.u, hit_record.v, &hit_record.location),
            scattered: Ray {
                origin: hit_record.location,
                direction: Point64(random_in_unit_sphere()),
                exposure_time: ray_in.exposure_time,
            },
        })
    }
}
