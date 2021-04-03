use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::point64::Point64;
use crate::ray::Ray;
use std::sync::Arc;

#[derive(Clone)]
pub struct MovingSphere {
    pub center0: Point64,
    pub center1: Point64,
    pub radius: f64,
    pub material: Arc<dyn Material + Send + Sync>,
    pub time0: f64,
    pub time1: f64,
}

impl MovingSphere {
    pub fn center_at(&self, time: f64) -> Point64 {
        Point64(*self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (*self.center1 - *self.center0))
    }
}

impl Hittable for MovingSphere {
    fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord> {
        let ray_origin_to_center = *ray.origin - *self.center_at(ray.exposure_time);
        let a = ray.direction.dot(&ray.direction);
        let half_b = ray_origin_to_center.dot(&ray.direction);
        let c = ray_origin_to_center.dot(&ray_origin_to_center) - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        let sqrt_discriminant = discriminant.sqrt();
        let root_one = (-half_b - sqrt_discriminant) / a;
        let root_two = (-half_b + sqrt_discriminant) / a;

        if discriminant >= 0.0 {
            let root_one_in_range = min_value < root_one && root_one < max_value;
            let root_two_in_range = min_value < root_two && root_two < max_value;

            if root_one_in_range || root_two_in_range {
                let root = if root_one_in_range {
                    root_one
                } else {
                    root_two
                };

                let location = ray.point_at_parameter(root);
                let outward_normal = Point64((*location - *self.center_at(ray.exposure_time)) / self.radius);

                Some(HitRecord::new(root, ray, outward_normal, &self.material))
            } else {
                None
            }
        } else {
            None
        }
    }
}
