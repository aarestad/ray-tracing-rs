use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::hittable_vec::HittableVec;
use crate::hittables::quad::Quad;
use crate::hittables::{HitRecord, Hittable};
use crate::materials::Material;

#[derive(Clone)]
pub struct Cuboid {
    cuboid_min: Point64,
    cuboid_max: Point64,
    sides: HittableVec,
}

impl Cuboid {
    pub fn new(p0: Point64, p1: Point64, material: Material) -> Self {
        let dx = Point64::new(p1.x() - p0.x(), 0., 0.);
        let dy = Point64::new(0., p1.y() - p0.y(), 0.);
        let dz = Point64::new(0., 0., p1.z() - p0.z());

        Self {
            cuboid_min: p0,
            cuboid_max: p1,
            sides: HittableVec {
                hittables: vec![
                    // X+ face
                    Hittable::Quad(Quad::new(
                        Point64::new(p1.x(), p0.y(), p0.z()),
                        dy,
                        dz,
                        material.clone(),
                    )),
                    // X- face
                    Hittable::Quad(Quad::new(p0, dy, dz, material.clone())),
                    // Y+ face
                    Hittable::Quad(Quad::new(
                        Point64::new(p0.x(), p1.y(), p0.z()),
                        dx,
                        dz,
                        material.clone(),
                    )),
                    // Y- face
                    Hittable::Quad(Quad::new(p0, dx, dz, material.clone())),
                    // Z+ face
                    Hittable::Quad(Quad::new(
                        Point64::new(p0.x(), p0.y(), p1.z()),
                        dx,
                        dy,
                        material.clone(),
                    )),
                    // Z- face
                    Hittable::Quad(Quad::new(p0, dx, dy, material)),
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
