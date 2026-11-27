use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::data::vector3::random_unit_vector;
use crate::hittables::HitRecord;
use crate::materials::ScatterRecord;
use crate::textures::Texture;

#[derive(Clone)]
pub struct Isotropic {
    pub albedo: Texture,
}

impl Isotropic {
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            hit_record: hit_record.clone(),
            attenuation: self
                .albedo
                .value(hit_record.u, hit_record.v, &hit_record.location),
            scattered: Ray {
                origin: hit_record.location,
                direction: Point64(random_unit_vector()),
                exposure_time: ray_in.exposure_time,
            },
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::color64::Color64;
    use crate::materials::Material;
    use approx::assert_abs_diff_eq;

    #[test]
    fn scatters_uniformly_in_all_directions() {
        let iso = Isotropic {
            albedo: Texture::solid(Color64::new(0.2, 0.4, 0.6)),
        };
        let ray = Ray {
            origin: Point64::new(0., 0., 5.),
            direction: Point64::new(0., 0., -1.),
            exposure_time: 0.25,
        };
        let hr = HitRecord::new(
            5.,
            &ray,
            Point64::new(1., 0., 0.),
            Material::Isotropic(iso.clone()),
            (0., 0.),
        );
        let n = 20_000;
        let mut sum = Point64::new(0., 0., 0.).0;
        for _ in 0..n {
            let sr = iso.scatter(&ray, &hr).unwrap();
            assert_eq!(sr.attenuation, Color64::new(0.2, 0.4, 0.6));
            assert_eq!(sr.scattered.exposure_time, 0.25);
            assert_abs_diff_eq!(sr.scattered.direction.0.magnitude(), 1., epsilon = 1e-12);
            sum += sr.scattered.direction.0;
        }
        // Uniform on the sphere: mean direction ≈ 0 (std. error ≈ 0.004 per axis).
        assert!((sum / n as f64).magnitude() < 0.03);
    }
}
