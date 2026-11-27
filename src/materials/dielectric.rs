use crate::data::color64::Color64;
use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::data::vector3::{reflect, refract};
use crate::hittables::HitRecord;
use crate::materials::ScatterRecord;
use rand_distr::num_traits::Inv;

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0.powi(2);
    r0 + (1. - r0) * (1. - cosine).powi(5)
}

#[derive(Clone)]
pub struct Dielectric {
    pub index_of_refraction: f64,
}

impl Dielectric {
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let refraction_ratio = if hit_record.front_face {
            self.index_of_refraction.inv()
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray_in.direction.0.normalize();

        let cos_theta = (-unit_direction.dot(&hit_record.normal.0)).min(1.);
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.;

        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand::random() {
                reflect(&unit_direction, &hit_record.normal.0)
            } else {
                refract(&unit_direction, &hit_record.normal.0, refraction_ratio)
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::point64::Point64;
    use crate::materials::Material;
    use approx::assert_abs_diff_eq;

    #[test]
    fn schlick_endpoints() {
        // Head-on: r0 = ((1 - n) / (1 + n))^2 = 0.04 for n = 1.5.
        assert_abs_diff_eq!(reflectance(1.0, 1.5), 0.04, epsilon = 1e-12);
        // Grazing: everything reflects.
        assert_abs_diff_eq!(reflectance(0.0, 1.5), 1.0, epsilon = 1e-12);
    }

    #[test]
    fn total_internal_reflection_always_reflects() {
        let glass = Dielectric {
            index_of_refraction: 1.5,
        };
        // Ray inside the glass (back face) at 60° from the normal: 1.5 * sin 60° > 1.
        let theta = 60f64.to_radians();
        let ray = Ray {
            origin: Point64::new(0., -1., 0.),
            direction: Point64::new(theta.sin(), theta.cos(), 0.),
            exposure_time: 0.,
        };
        // Outward normal points +y, same way as the ray, so this is a back-face hit.
        let hr = HitRecord::new(
            1.,
            &ray,
            Point64::new(0., 1., 0.),
            Material::Dielectric(glass.clone()),
            (0., 0.),
        );
        assert!(!hr.front_face);
        for _ in 0..100 {
            let out = glass.scatter(&ray, &hr).unwrap().scattered.direction.0;
            assert_abs_diff_eq!(out.x, theta.sin(), epsilon = 1e-9);
            assert_abs_diff_eq!(out.y, -theta.cos(), epsilon = 1e-9);
        }
    }

    #[test]
    fn head_on_refraction_goes_straight_through() {
        let glass = Dielectric {
            index_of_refraction: 1.5,
        };
        let ray = Ray {
            origin: Point64::new(0., 1., 0.),
            direction: Point64::new(0., -1., 0.),
            exposure_time: 0.,
        };
        let hr = HitRecord::new(
            1.,
            &ray,
            Point64::new(0., 1., 0.),
            Material::Dielectric(glass.clone()),
            (0., 0.),
        );
        // ~4% of samples reflect (Schlick); the rest must continue straight down.
        for _ in 0..100 {
            let out = glass.scatter(&ray, &hr).unwrap().scattered.direction.0;
            assert!(out.y.abs() > 1. - 1e-9 && out.x.abs() < 1e-9);
        }
    }
}
