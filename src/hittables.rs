use std::sync::Arc;

use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::materials::Material;

pub mod axis_aligned_bounding_box;
pub mod bounded_volume_hierarchy;
pub mod hittable_vec;
pub mod moving_sphere;
pub mod sphere;

#[derive(Clone)]
pub struct HitRecord {
    pub value: f64,
    pub location: Point64,
    pub normal: Point64,
    pub front_face: bool,
    pub material: Arc<dyn Material + Send + Sync>,
}

impl HitRecord {
    pub fn new(
        value: f64,
        ray: &Ray,
        outward_normal: Point64,
        material: &Arc<dyn Material + Send + Sync>,
    ) -> HitRecord {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;

        let normal = if front_face {
            outward_normal
        } else {
            Point64(-*outward_normal)
        };

        HitRecord {
            value,
            location: ray.point_at_parameter(value),
            normal,
            front_face,
            material: material.clone(),
        }
    }
}

pub trait Hittable: Sync {
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AxisAlignedBoundingBox>;

    fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord>;
}
