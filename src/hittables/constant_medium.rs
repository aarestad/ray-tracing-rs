use crate::data::interval::Interval;
use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::{HitRecord, Hittable};
use crate::materials::Material;
use rand::Rng;

#[derive(Clone)]
pub struct ConstantMedium {
    pub boundary: Box<Hittable>,
    pub neg_inv_density: f64,
    pub phase_function: Material,
}

impl ConstantMedium {
    pub fn new(boundary: Box<Hittable>, density: f64, phase_function: Material) -> Self {
        ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function,
        }
    }

    pub fn bounding_box(&self, time0: f64, time1: f64) -> Option<AxisAlignedBoundingBox> {
        self.boundary.bounding_box(time0, time1)
    }

    pub fn is_hit_by(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let rec1 = self.boundary.is_hit_by(ray, Interval::UNIVERSE)?;
        let rec2 = self
            .boundary
            .is_hit_by(ray, Interval::new(rec1.value + 0.0001, f64::INFINITY))?;

        let t_min = rec1.value.max(ray_t.min);
        let t_max = rec2.value.min(ray_t.max);

        if t_min >= t_max {
            return None;
        }

        let t_min = t_min.max(0.0);

        let ray_length = ray.direction.0.magnitude();
        let distance_inside_boundary = (t_max - t_min) * ray_length;
        let hit_distance = self.neg_inv_density * rand::rng().random::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = t_min + hit_distance / ray_length;
        let location = ray.point_at_parameter(t);

        Some(HitRecord {
            value: t,
            u: 0.0,
            v: 0.0,
            location,
            normal: Point64::new(1.0, 0.0, 0.0), // arbitrary
            front_face: true,                    // arbitrary
            material: self.phase_function.clone(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::color64::Color64;
    use crate::hittables::sphere::Sphere;
    use crate::materials::Isotropic;
    use crate::materials::dielectric::Dielectric;
    use crate::textures::Texture;
    use approx::assert_abs_diff_eq;

    /// Medium filling a sphere of radius 10 centered at the origin.
    fn fog(density: f64) -> ConstantMedium {
        ConstantMedium::new(
            Box::new(Hittable::Sphere(Sphere {
                center: Point64::new(0., 0., 0.),
                radius: 10.,
                material: Material::Dielectric(Dielectric {
                    index_of_refraction: 1.5,
                }),
            })),
            density,
            Material::Isotropic(Isotropic {
                albedo: Texture::solid(Color64::new(0.5, 0.5, 0.5)),
            }),
        )
    }

    /// Ray from z = 20 toward the origin; it enters the sphere at t = 10, exits at t = 30.
    fn ray_from_outside(dir_len: f64) -> Ray {
        Ray {
            origin: Point64::new(0., 0., 20.),
            direction: Point64::new(0., 0., -dir_len),
            exposure_time: 0.,
        }
    }

    const RANGE: Interval = Interval::new(0.001, f64::INFINITY);

    #[test]
    fn dense_medium_scatters_at_the_boundary() {
        let m = fog(1e9);
        for _ in 0..100 {
            let hr = m.is_hit_by(&ray_from_outside(1.), RANGE).unwrap();
            assert_abs_diff_eq!(hr.value, 10., epsilon = 1e-6);
            assert_abs_diff_eq!(hr.location.z(), 10., epsilon = 1e-6);
        }
    }

    #[test]
    fn thin_medium_is_almost_always_passed_through() {
        let m = fog(1e-9);
        assert!((0..1000).all(|_| m.is_hit_by(&ray_from_outside(1.), RANGE).is_none()));
    }

    #[test]
    fn mean_free_path_is_one_over_density_in_world_units() {
        // Distance 20 inside the sphere; with density 0.5, about 1 - e^-10 of rays scatter,
        // at a mean depth of 1 / 0.5 = 2 world units past the entry point. A non-unit
        // direction must not change the distance (t is scaled by the direction's length).
        let m = fog(0.5);
        for dir_len in [1., 4.] {
            let n = 20_000;
            let total: f64 = (0..n)
                .filter_map(|_| m.is_hit_by(&ray_from_outside(dir_len), RANGE))
                .map(|hr| 10. - hr.location.z())
                .sum();
            let mean = total / n as f64;
            assert!(
                (mean - 2.).abs() < 0.1,
                "mean free path {mean} for |dir| = {dir_len}"
            );
        }
    }

    #[test]
    fn ray_starting_inside_scatters_ahead_of_origin() {
        let m = fog(1e9);
        let r = Ray {
            origin: Point64::new(0., 0., 0.),
            direction: Point64::new(1., 0., 0.),
            exposure_time: 0.,
        };
        let hr = m.is_hit_by(&r, RANGE).unwrap();
        // Entry is clamped to the start of ray_t, not the boundary behind the ray at t = -10.
        assert!(hr.value >= RANGE.min && hr.value < RANGE.min + 1e-6);
    }

    #[test]
    fn respects_ray_t_max() {
        // A closer object already capped the search before the medium starts.
        let m = fog(1e9);
        assert!(
            m.is_hit_by(&ray_from_outside(1.), Interval::new(0.001, 5.))
                .is_none()
        );
    }

    #[test]
    fn ray_missing_boundary_misses() {
        let r = Ray {
            origin: Point64::new(50., 0., 20.),
            direction: Point64::new(0., 0., -1.),
            exposure_time: 0.,
        };
        assert!(fog(1e9).is_hit_by(&r, RANGE).is_none());
    }
}
