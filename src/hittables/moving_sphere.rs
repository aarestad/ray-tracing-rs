use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::sphere::get_sphere_uv;
use crate::hittables::{HitRecord, Hittable};
use crate::materials::Material;
use nalgebra::Vector3;
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
        self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }
}

impl Hittable for MovingSphere {
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AxisAlignedBoundingBox> {
        let half_box_side = Vector3::new(self.radius, self.radius, self.radius);
        let center0 = self.center_at(time0);
        let center1 = self.center_at(time1);

        let box0 = AxisAlignedBoundingBox {
            minimum: Point64(center0.0 - half_box_side),
            maximum: Point64(center0.0 + half_box_side),
        };

        let box1 = AxisAlignedBoundingBox {
            minimum: Point64(center1.0 - half_box_side),
            maximum: Point64(center1.0 + half_box_side),
        };

        Some(box0.surrounding_box_with(&box1))
    }

    fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord> {
        let ray_origin_to_center = ray.origin - self.center_at(ray.exposure_time);
        let a = ray.direction.0.dot(&ray.direction.0);
        let half_b = ray_origin_to_center.0.dot(&ray.direction.0);
        let c = ray_origin_to_center.0.dot(&ray_origin_to_center.0) - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        let sqrt_discriminant = discriminant.sqrt();
        let root_one = (-half_b - sqrt_discriminant) / a;
        let root_two = (-half_b + sqrt_discriminant) / a;

        if discriminant >= 0. {
            let root_one_in_range = min_value < root_one && root_one < max_value;
            let root_two_in_range = min_value < root_two && root_two < max_value;

            if root_one_in_range || root_two_in_range {
                let root = if root_one_in_range {
                    root_one
                } else {
                    root_two
                };

                let location = ray.point_at_parameter(root);
                let outward_normal = (location - self.center_at(ray.exposure_time)) / self.radius;

                Some(HitRecord::new(
                    root,
                    ray,
                    outward_normal,
                    self.material.clone(),
                    get_sphere_uv(outward_normal),
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}
