use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::HitRecord;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::materials::Material;
use nalgebra::Vector3;
use std::f64::consts::{PI, TAU};

#[derive(Clone)]
pub struct Sphere {
    pub center: Point64,
    pub radius: f64,
    pub material: Material,
}

pub(crate) fn get_sphere_uv(p: Point64) -> (f64, f64) {
    let theta = (-p.y()).acos();
    let phi = (-p.z()).atan2(p.x()) + PI;

    (phi / TAU, theta / PI)
}

impl Sphere {
    pub fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AxisAlignedBoundingBox> {
        Some(AxisAlignedBoundingBox {
            minimum: Point64(self.center.0 - Vector3::new(self.radius, self.radius, self.radius)),
            maximum: Point64(self.center.0 + Vector3::new(self.radius, self.radius, self.radius)),
        })
    }

    pub fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord> {
        let ray_origin_to_center: Point64 = ray.origin - self.center;
        let a = ray.direction.0.dot(&ray.direction.0);
        let half_b = ray_origin_to_center.0.dot(&ray.direction.0);
        let c = ray_origin_to_center.0.dot(&ray_origin_to_center.0) - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant >= 0. {
            let sqrt_discriminant = discriminant.sqrt();
            let root_one = (-half_b - sqrt_discriminant) / a;
            let root_two = (-half_b + sqrt_discriminant) / a;
            let root_one_in_range = min_value < root_one && root_one < max_value;
            let root_two_in_range = min_value < root_two && root_two < max_value;

            if root_one_in_range || root_two_in_range {
                let root = if root_one_in_range {
                    root_one
                } else {
                    root_two
                };

                let location = ray.point_at_parameter(root);
                let outward_normal = (location - self.center) / self.radius;

                return Some(HitRecord::new(
                    root,
                    ray,
                    outward_normal,
                    self.material.clone(),
                    get_sphere_uv(outward_normal),
                ));
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::color64::Color64;
    use crate::materials::diffuse_light::DiffuseLight;
    use approx::assert_abs_diff_eq;

    fn unit_sphere_at_neg_z() -> Sphere {
        Sphere {
            center: Point64::new(0., 0., -1.),
            radius: 0.5,
            material: Material::DiffuseLight(DiffuseLight::new(Color64::new(1., 0., 0.))),
        }
    }

    fn ray(origin: Point64, direction: Point64) -> Ray {
        Ray {
            origin,
            direction,
            exposure_time: 0.,
        }
    }

    #[test]
    fn hits_near_side_from_outside() {
        let s = unit_sphere_at_neg_z();
        let hit = s
            .is_hit_by(&ray(Point64::new(0., 0., 0.), Point64::new(0., 0., -1.)), 0.001, f64::INFINITY)
            .expect("ray toward center should hit");
        assert_abs_diff_eq!(hit.value, 0.5, epsilon = 1e-12);
        assert_abs_diff_eq!(hit.location.0, Point64::new(0., 0., -0.5).0, epsilon = 1e-12);
        assert!(hit.front_face);
        assert_abs_diff_eq!(hit.normal.0, Point64::new(0., 0., 1.).0, epsilon = 1e-12);
    }

    #[test]
    fn non_normalized_direction_gives_scaled_t() {
        let s = unit_sphere_at_neg_z();
        let hit = s
            .is_hit_by(&ray(Point64::new(0., 0., 0.), Point64::new(0., 0., -2.)), 0.001, f64::INFINITY)
            .unwrap();
        assert_abs_diff_eq!(hit.value, 0.25, epsilon = 1e-12);
    }

    #[test]
    fn misses() {
        let s = unit_sphere_at_neg_z();
        assert!(s
            .is_hit_by(&ray(Point64::new(0., 0., 0.), Point64::new(0., 1., -1.)), 0.001, f64::INFINITY)
            .is_none());
        // Sphere is behind the ray.
        assert!(s
            .is_hit_by(&ray(Point64::new(0., 0., 0.), Point64::new(0., 0., 1.)), 0.001, f64::INFINITY)
            .is_none());
    }

    #[test]
    fn from_inside_hits_far_side_as_back_face() {
        let s = unit_sphere_at_neg_z();
        let hit = s
            .is_hit_by(&ray(Point64::new(0., 0., -1.), Point64::new(0., 0., -1.)), 0.001, f64::INFINITY)
            .unwrap();
        assert_abs_diff_eq!(hit.value, 0.5, epsilon = 1e-12);
        assert!(!hit.front_face);
        // Normal is flipped to oppose the ray.
        assert_abs_diff_eq!(hit.normal.0, Point64::new(0., 0., 1.).0, epsilon = 1e-12);
    }

    #[test]
    fn respects_t_range() {
        let s = unit_sphere_at_neg_z();
        let r = ray(Point64::new(0., 0., 0.), Point64::new(0., 0., -1.));
        // Near root (0.5) excluded -> far root (1.5).
        assert_abs_diff_eq!(s.is_hit_by(&r, 0.6, f64::INFINITY).unwrap().value, 1.5, epsilon = 1e-12);
        // Both roots excluded.
        assert!(s.is_hit_by(&r, 0.001, 0.4).is_none());
    }
}
