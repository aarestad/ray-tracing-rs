use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::axis_aligned_rect::{AxisAlignedRect, AxisAlignment};
use crate::hittables::hittable_vec::HittableVec;
use crate::hittables::{HitRecord, Hittable};
use crate::materials::Material;

#[derive(Clone)]
pub struct Cuboid {
    cuboid_min: Point64,
    cuboid_max: Point64,
    sides: HittableVec,
}

impl Cuboid {
    pub fn new(cuboid_min: Point64, cuboid_max: Point64, material: Material) -> Self {
        Self {
            cuboid_min,
            cuboid_max,
            sides: HittableVec {
                hittables: vec![
                    Hittable::AxisAlignedRect(AxisAlignedRect {
                        material: material.clone(),
                        min: (cuboid_min.y(), cuboid_min.z()),
                        max: (cuboid_max.y(), cuboid_max.z()),
                        axis_value: cuboid_max.x(),
                        axis_alignment: AxisAlignment::X,
                    }),
                    Hittable::AxisAlignedRect(AxisAlignedRect {
                        material: material.clone(),
                        min: (cuboid_min.y(), cuboid_min.z()),
                        max: (cuboid_max.y(), cuboid_max.z()),
                        axis_value: cuboid_min.x(),
                        axis_alignment: AxisAlignment::X,
                    }),
                    Hittable::AxisAlignedRect(AxisAlignedRect {
                        material: material.clone(),
                        min: (cuboid_min.x(), cuboid_min.z()),
                        max: (cuboid_max.x(), cuboid_max.z()),
                        axis_value: cuboid_max.y(),
                        axis_alignment: AxisAlignment::Y,
                    }),
                    Hittable::AxisAlignedRect(AxisAlignedRect {
                        material: material.clone(),
                        min: (cuboid_min.x(), cuboid_min.z()),
                        max: (cuboid_max.x(), cuboid_max.z()),
                        axis_value: cuboid_min.y(),
                        axis_alignment: AxisAlignment::Y,
                    }),
                    Hittable::AxisAlignedRect(AxisAlignedRect {
                        material: material.clone(),
                        min: (cuboid_min.x(), cuboid_min.y()),
                        max: (cuboid_max.x(), cuboid_max.y()),
                        axis_value: cuboid_max.z(),
                        axis_alignment: AxisAlignment::Z,
                    }),
                    Hittable::AxisAlignedRect(AxisAlignedRect {
                        material,
                        min: (cuboid_min.x(), cuboid_min.y()),
                        max: (cuboid_max.x(), cuboid_max.y()),
                        axis_value: cuboid_min.z(),
                        axis_alignment: AxisAlignment::Z,
                    }),
                ],
            },
        }
    }

    pub fn bounding_box(&self, _: f64, _: f64) -> Option<AxisAlignedBoundingBox> {
        Some(AxisAlignedBoundingBox {
            minimum: self.cuboid_min,
            maximum: self.cuboid_max,
        })
    }

    pub fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord> {
        self.sides.is_hit_by(ray, min_value, max_value)
    }
}
