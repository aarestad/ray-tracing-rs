use crate::color64::Color64;
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::point64::Point64;
use crate::ray::Ray;

pub struct Dielectric {
    pub index_of_refraction: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray_in.direction.normalized();

        let cos_theta = -unit_direction.dot(&*hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract {
            unit_direction.reflect(&*hit_record.normal)
        } else {
            unit_direction.refract(&*hit_record.normal, refraction_ratio)
        };

        Some(ScatterRecord {
            hit_record: hit_record.clone(),
            attenuation: Color64::new(1.0, 1.0, 1.0),
            scattered: Ray {
                origin: hit_record.location,
                direction: Point64(direction),
            },
        })
    }
}
