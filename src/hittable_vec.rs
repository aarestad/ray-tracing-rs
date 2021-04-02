use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableVec {
    pub hittables: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableVec {
    fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord> {
        let mut winner = None;

        for hittable in self.hittables.iter() {
            let result = hittable.is_hit_by(
                ray,
                min_value,
                winner.as_ref().map_or(max_value, |hr: &HitRecord| hr.value),
            );

            if result.is_some() {
                winner = result;
            }
        }

        winner
    }
}
