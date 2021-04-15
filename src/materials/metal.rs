use crate::data::color64::Color64;
use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::HitRecord;
use crate::materials::{Material, ScatterRecord};
use crate::data::vector3::{random_in_unit_sphere, reflect};

pub struct Metal {
    pub albedo: Color64,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(ray_in.direction.normalize(), &hit_record.normal);

        if reflected.dot(&hit_record.normal) > 0. {
            Some(ScatterRecord {
                hit_record: hit_record.clone(),
                attenuation: self.albedo,
                scattered: Ray {
                    origin: hit_record.location,
                    direction: Point64(reflected + self.fuzz * random_in_unit_sphere()),
                    exposure_time: ray_in.exposure_time,
                },
            })
        } else {
            None
        }
    }
}
