use std::sync::Arc;

use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::materials::Material;

mod axis_aligned_bounding_box;
pub mod axis_aligned_rect;
pub mod bounded_volume_hierarchy;
mod bvh_comparators;
pub mod cuboid;
pub mod hittable_vec;
pub mod moving_sphere;
pub mod rotation;
pub mod sphere;
pub mod translation;
pub mod triangle;

#[derive(Clone)]
pub struct HitRecord {
    pub value: f64,
    pub u: f64,
    pub v: f64,
    pub location: Point64,
    pub normal: Point64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        value: f64,
        ray: &Ray,
        outward_normal: Point64,
        material: Arc<dyn Material>,
        uv: (f64, f64),
    ) -> HitRecord {
        let front_face = ray.direction.0.dot(&outward_normal.0) < 0.;

        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            value,
            u: uv.0,
            v: uv.1,
            location: ray.point_at_parameter(value),
            normal,
            front_face,
            material,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AxisAlignedBoundingBox>;

    fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord>;
}
