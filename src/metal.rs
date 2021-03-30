use crate::color64::Color64;
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::point64::Point64;
use crate::ray::Ray;

pub struct Metal {
    pub albedo: Color64,
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
                    direction: Point64(reflected),
                },
            })
        } else {
            None
        }
    }
}
