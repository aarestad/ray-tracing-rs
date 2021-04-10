use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::data::vec3_64::Vec3_64;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::{HitRecord, Hittable};
use crate::materials::Material;
use std::sync::Arc;

#[derive(Clone)]
pub struct MovingSphere {
    pub center0: Point64,
    pub center1: Point64,
    pub radius: f64,
    pub material: Arc<dyn Material>,
    pub time0: f64,
    pub time1: f64,
}

impl MovingSphere {
    pub fn center_at(&self, time: f64) -> Point64 {
        Point64(
            *self.center0
                + ((time - self.time0) / (self.time1 - self.time0))
                    * (*self.center1 - *self.center0),
        )
    }
}

impl Hittable for MovingSphere {
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AxisAlignedBoundingBox> {
        let half_box_side = Vec3_64(self.radius, self.radius, self.radius);
        let center0 = *self.center_at(time0);
        let center1 = *self.center_at(time1);

        let box0 = AxisAlignedBoundingBox {
            minimum: Point64(center0 - half_box_side),
            maximum: Point64(center0 + half_box_side),
        };

        let box1 = AxisAlignedBoundingBox {
            minimum: Point64(center1 - half_box_side),
            maximum: Point64(center1 + half_box_side),
        };

        Some(box0.surrounding_box_with(&box1))
    }

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
                let outward_normal =
                    Point64((*location - *self.center_at(ray.exposure_time)) / self.radius);

                Some(HitRecord::new(
                    root,
                    ray,
                    outward_normal,
                    &self.material,
                    Default::default(),
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}
