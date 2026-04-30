use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::{HitRecord, Hittable};
use crate::materials::Material;
use rand::Rng;

#[derive(Clone)]
pub struct ConstantMedium {
    pub boundary: Box<Hittable>,
    pub neg_inv_density: f64,
    pub phase_function: Material,
}

impl ConstantMedium {
    pub fn new(boundary: Box<Hittable>, density: f64, phase_function: Material) -> Self {
        ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function,
        }
    }

    pub fn bounding_box(&self, time0: f64, time1: f64) -> Option<AxisAlignedBoundingBox> {
        self.boundary.bounding_box(time0, time1)
    }

    pub fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord> {
        let rec1 = self
            .boundary
            .is_hit_by(ray, f64::NEG_INFINITY, f64::INFINITY)?;
        let rec2 = self
            .boundary
            .is_hit_by(ray, rec1.value + 0.0001, f64::INFINITY)?;

        let t_min = rec1.value.max(min_value);
        let t_max = rec2.value.min(max_value);

        if t_min >= t_max {
            return None;
        }

        let t_min = t_min.max(0.0);

        let ray_length = ray.direction.0.magnitude();
        let distance_inside_boundary = (t_max - t_min) * ray_length;
        let hit_distance = self.neg_inv_density * rand::rng().random::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = t_min + hit_distance / ray_length;
        let location = ray.point_at_parameter(t);

        Some(HitRecord {
            value: t,
            u: 0.0,
            v: 0.0,
            location,
            normal: Point64::new(1.0, 0.0, 0.0), // arbitrary
            front_face: true,                    // arbitrary
            material: self.phase_function.clone(),
        })
    }
}
