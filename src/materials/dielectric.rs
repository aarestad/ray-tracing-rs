use crate::data::color64::Color64;
use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::HitRecord;
use crate::materials::{Material, ScatterRecord};

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0.powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

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

        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand::random() {
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
                exposure_time: ray_in.exposure_time,
            },
        })
    }
}
