use crate::color64::Color64;
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::point64::Point64;
use crate::ray::Ray;
use crate::vec3_64::Vec3_64;

pub struct Lambertian {
    pub color: Color64,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let scatter_direction = *hit_record.normal + Vec3_64::random_unit_vector();

        Some(ScatterRecord {
            hit_record: hit_record.clone(),
            attenuation: self.color,

            scattered: Ray {
                origin: hit_record.location,
                direction: if scatter_direction.near_zero() {
                    hit_record.normal
                } else {
                    Point64(scatter_direction)
                },
            },
        })
    }
}
