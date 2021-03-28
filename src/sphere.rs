use crate::point64::Point64;
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};

pub struct Sphere {
    pub center: Point64,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord> {
        let ray_origin_to_center = **ray.origin - *self.center;
        let a = ray.direction.dot(ray.direction);
        let b = ray_origin_to_center.dot(ray.direction);
        let c = ray_origin_to_center.dot(&ray_origin_to_center) - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;

        let sqrt_discriminant = discriminant.sqrt();
        let root_one = (-b - sqrt_discriminant) / a;
        let root_two = (-b + sqrt_discriminant) / a;

        if discriminant >= 0.0 {
            let root_one_in_range = min_value < root_one && root_one < max_value;
            let root_two_in_range = min_value < root_two && root_two < max_value;

            if root_one_in_range || root_two_in_range {
                let value = if root_one_in_range { root_one } else { root_two };
                let location = ray.point_at_parameter(value);

                Some(HitRecord {
                    value,
                    location,
                    normal: Point64((*location - *self.center) / self.radius)
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
