use crate::material::Material;
use crate::point64::Point64;
use crate::ray::Ray;
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub value: f64,
    pub location: Point64,
    pub normal: Point64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        value: f64,
        ray: &Ray,
        outward_normal: Point64,
        material: &Rc<dyn Material>,
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

pub trait Hittable {
    fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord>;
}
