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

use axis_aligned_rect::AxisAlignedRect;
use bounded_volume_hierarchy::BoundedVolumeHierarchy;
use cuboid::Cuboid;
use hittable_vec::HittableVec;
use moving_sphere::MovingSphere;
use rotation::Rotation;
use sphere::Sphere;
use translation::Translation;
use triangle::Triangle;

#[derive(Clone)]
pub struct HitRecord {
    pub value: f64,
    pub u: f64,
    pub v: f64,
    pub location: Point64,
    pub normal: Point64,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn new(
        value: f64,
        ray: &Ray,
        outward_normal: Point64,
        material: Material,
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

#[allow(dead_code)]
#[derive(Clone)]
pub enum Hittable {
    Sphere(Sphere),
    MovingSphere(MovingSphere),
    AxisAlignedRect(AxisAlignedRect),
    Triangle(Triangle),
    HittableVec(HittableVec),
    Bvh(BoundedVolumeHierarchy),
    Translation(Translation),
    Cuboid(Cuboid),
    Rotation(Rotation),
}

impl Hittable {
    pub fn bounding_box(&self, time0: f64, time1: f64) -> Option<AxisAlignedBoundingBox> {
        match self {
            Hittable::Sphere(h) => h.bounding_box(time0, time1),
            Hittable::MovingSphere(h) => h.bounding_box(time0, time1),
            Hittable::AxisAlignedRect(h) => h.bounding_box(time0, time1),
            Hittable::Triangle(h) => h.bounding_box(time0, time1),
            Hittable::HittableVec(h) => h.bounding_box(time0, time1),
            Hittable::Bvh(h) => h.bounding_box(time0, time1),
            Hittable::Translation(h) => h.bounding_box(time0, time1),
            Hittable::Cuboid(h) => h.bounding_box(time0, time1),
            Hittable::Rotation(h) => h.bounding_box(time0, time1),
        }
    }

    pub fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord> {
        match self {
            Hittable::Sphere(h) => h.is_hit_by(ray, min_value, max_value),
            Hittable::MovingSphere(h) => h.is_hit_by(ray, min_value, max_value),
            Hittable::AxisAlignedRect(h) => h.is_hit_by(ray, min_value, max_value),
            Hittable::Triangle(h) => h.is_hit_by(ray, min_value, max_value),
            Hittable::HittableVec(h) => h.is_hit_by(ray, min_value, max_value),
            Hittable::Bvh(h) => h.is_hit_by(ray, min_value, max_value),
            Hittable::Translation(h) => h.is_hit_by(ray, min_value, max_value),
            Hittable::Cuboid(h) => h.is_hit_by(ray, min_value, max_value),
            Hittable::Rotation(h) => h.is_hit_by(ray, min_value, max_value),
        }
    }
}
