use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::data::vector3::Vector;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::materials::Materials;
use nalgebra::Vector3;
use std::sync::Arc;

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
    pub material: Materials,
}

impl HitRecord {
    pub fn new(
        value: f64,
        ray: &Ray,
        outward_normal: Point64,
        material: Materials,
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

pub enum AxisAlignment {
    X,
    Y,
    Z,
}

#[derive()]
pub enum Hittables {
    AxisAlignedBoundingBox(Point64, Point64),
    AxisAlignedRect(Materials, (f64, f64), (f64, f64), f64, AxisAlignment),
    BoundedVolumeHierarchy(Box<Hittables>, Box<Hittables>, AxisAlignedBoundingBox),
    Cuboid(
        Point64,
        Point64,
        Box<Hittables>, /* should be a HittableVec */
    ),
    HittableVec(Vec<Box<Hittables>>),
    //     pub center0: Point64,
    //     pub center1: Point64,
    //     pub radius: f64,
    //     pub material: Materials,
    //     pub time0: f64,
    //     pub time1: f64,
    MovingSphere(Point64, Point64, f64, Materials, f64, f64),
    Rotation(
        Box<Hittables>,
        AxisAlignment,
        (f64, f64),
        Option<Box<Hittables>>,
    ),
    Sphere(Point64, f64, Materials),
    Translation(Box<Hittables>, Vector3<f64>),
    Triangle(Point64, Point64, Point64, Vector, Vector, Vector, Materials),
}

impl Hittables {
    pub fn bounding_box(&self, time0: f64, time1: f64) -> Option<AxisAlignedBoundingBox> {
        None
    }

    pub fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord> {
        None
    }
}
