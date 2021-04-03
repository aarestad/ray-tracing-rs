use crate::data::color64::Color64;
use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::data::vec3_64::Vec3_64;
use crate::hittables::HitRecord;
use crate::materials::{Material, ScatterRecord};

pub struct Metal {
    pub albedo: Color64,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected = ray_in.direction.normalized().reflect(&hit_record.normal);

        if reflected.dot(&hit_record.normal) > 0.0 {
            Some(ScatterRecord {
                hit_record: hit_record.clone(),
                attenuation: self.albedo,
                scattered: Ray {
                    origin: hit_record.location,
                    direction: Point64(reflected + self.fuzz * Vec3_64::random_in_unit_sphere()),
                    exposure_time: ray_in.exposure_time,
                },
            })
        } else {
            None
        }
    }
}