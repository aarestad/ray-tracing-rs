use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::{HitRecord, Hittable};
use crate::materials::Material;
use std::sync::Arc;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::data::vec3_64::Vec3_64;

#[derive(Clone)]
pub struct Sphere {
    pub center: Point64,
    pub radius: f64,
    pub material: Arc<dyn Material + Send + Sync>,
}

impl Hittable for Sphere {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AxisAlignedBoundingBox> {
        Some(AxisAlignedBoundingBox {
            minimum: Point64(*self.center - Vec3_64(self.radius, self.radius, self.radius)),
            maximum: Point64(*self.center + Vec3_64(self.radius, self.radius, self.radius))
        })
    }

    fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord> {
        let ray_origin_to_center = *ray.origin - *self.center;
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
                let outward_normal = Point64((*location - *self.center) / self.radius);

                Some(HitRecord::new(root, ray, outward_normal, &self.material))
            } else {
                None
            }
        } else {
            None
        }
    }
}
