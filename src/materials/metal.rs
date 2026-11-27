use crate::data::color64::Color64;
use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::data::vector3::{random_unit_vector, reflect};
use crate::hittables::HitRecord;
use crate::materials::ScatterRecord;

#[derive(Clone)]
pub struct Metal {
    pub albedo: Color64,
    pub fuzz: f64,
}

impl Metal {
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(&ray_in.direction.0.normalize(), &hit_record.normal.0);
        // Book 10.6: fuzz the reflected direction (fuzz capped at 1), then absorb anything
        // that ends up pointing into the surface.
        let direction = reflected + self.fuzz.min(1.) * random_unit_vector();

        if direction.dot(&hit_record.normal.0) > 0. {
            Some(ScatterRecord {
                hit_record: hit_record.clone(),
                attenuation: self.albedo,
                scattered: Ray {
                    origin: hit_record.location,
                    direction: Point64(direction),
                    exposure_time: ray_in.exposure_time,
                },
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::materials::Material;
    use approx::assert_abs_diff_eq;

    fn grazing_hit(metal: &Metal) -> (Ray, HitRecord) {
        // Nearly grazing incidence so heavy fuzz frequently pushes rays below the surface.
        let ray = Ray {
            origin: Point64::new(-1., 0.05, 0.),
            direction: Point64::new(1., -0.05, 0.),
            exposure_time: 0.,
        };
        let hr = HitRecord::new(
            1.,
            &ray,
            Point64::new(0., 1., 0.),
            Material::Metal(metal.clone()),
            (0., 0.),
        );
        (ray, hr)
    }

    #[test]
    fn mirror_reflects_exactly() {
        let m = Metal {
            albedo: Color64::new(1., 1., 1.),
            fuzz: 0.,
        };
        let (ray, hr) = grazing_hit(&m);
        let out = m.scatter(&ray, &hr).unwrap().scattered.direction.0;
        let expected = Point64::new(1., 0.05, 0.).0.normalize();
        assert_abs_diff_eq!(out, expected, epsilon = 1e-12);
    }

    #[test]
    fn fuzzed_rays_never_point_into_surface() {
        let m = Metal {
            albedo: Color64::new(1., 1., 1.),
            fuzz: 5.0, // capped at 1
        };
        let (ray, hr) = grazing_hit(&m);
        let mut absorbed = 0;
        for _ in 0..1000 {
            match m.scatter(&ray, &hr) {
                Some(sr) => {
                    let d = sr.scattered.direction.0;
                    assert!(d.dot(&hr.normal.0) > 0.);
                    // Fuzz is capped at 1, so the offset from the mirror direction is at most 1.
                    let mirror = Point64::new(1., 0.05, 0.).0.normalize();
                    assert!((d - mirror).magnitude() <= 1. + 1e-9);
                }
                None => absorbed += 1,
            }
        }
        assert!(absorbed > 0, "grazing + heavy fuzz should absorb some rays");
    }
}
