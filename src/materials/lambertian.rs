use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::data::vector3::{near_zero, random_in_unit_sphere};
use crate::hittables::HitRecord;
use crate::materials::{Material, ScatterRecord};
use crate::textures::Texture;
use std::sync::Arc;

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let scatter_direction = hit_record.normal.0 + random_in_unit_sphere();

        Some(ScatterRecord {
            hit_record: hit_record.clone(),
            attenuation: self
                .albedo
                .value(hit_record.u, hit_record.v, &hit_record.location),
            scattered: Ray {
                origin: hit_record.location,
                direction: if near_zero(&scatter_direction) {
                    hit_record.normal
                } else {
                    Point64(scatter_direction)
                },
                exposure_time: ray_in.exposure_time,
            },
        })
    }
}
