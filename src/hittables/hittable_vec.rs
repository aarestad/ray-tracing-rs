use crate::data::ray::Ray;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::{HitRecord, Hittable};

#[derive(Clone)]
pub struct HittableVec {
    pub hittables: Vec<Hittable>,
}

impl HittableVec {
    pub fn bounding_box(&self, time0: f64, time1: f64) -> Option<AxisAlignedBoundingBox> {
        if self.hittables.is_empty() {
            return None;
        };

        self.hittables.iter().fold(
            self.hittables[0].bounding_box(time0, time1),
            |acc, hittable| Some(acc?.surrounding_box_with(&hittable.bounding_box(time0, time1)?)),
        )
    }

    pub fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord> {
        let mut winner: Option<HitRecord> = None;

        for hittable in self.hittables.iter() {
            let result = hittable.is_hit_by(
                ray,
                min_value,
                winner.as_ref().map_or(max_value, |hr| hr.value),
            );

            if result.is_some() {
                winner = result;
            }
        }

        winner
    }
}
