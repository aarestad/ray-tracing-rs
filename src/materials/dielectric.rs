use crate::data::color64::Color64;
use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::data::vector3::{reflect, refract};
use crate::hittables::HitRecord;
use crate::materials::{Material, ScatterRecord};
use rand_distr::num_traits::Inv;

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0.powi(2);
    r0 + (1. - r0) * (1. - cosine).powi(5)
}

pub struct Dielectric {
    pub index_of_refraction: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let refraction_ratio = if hit_record.front_face {
            self.index_of_refraction.inv()
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray_in.direction.normalize();

        let cos_theta = -unit_direction.dot(&*hit_record.normal).min(1.);
        let sin_theta = (1. - (cos_theta.powi(2) as f64)).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.;

        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand::random() {
                reflect(&unit_direction, &*hit_record.normal)
            } else {
                refract(&unit_direction, &*hit_record.normal, refraction_ratio)
            };

        Some(ScatterRecord {
            hit_record: hit_record.clone(),
            attenuation: Color64::new(1., 1., 1.),
            scattered: Ray {
                origin: hit_record.location,
                direction: Point64(direction),
                exposure_time: ray_in.exposure_time,
            },
        })
    }
}
