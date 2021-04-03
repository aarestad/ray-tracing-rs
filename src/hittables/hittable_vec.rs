use crate::data::ray::Ray;
use crate::hittables::{HitRecord, Hittable};

pub struct HittableVec {
    pub hittables: Vec<Box<dyn Hittable + Send + Sync>>,
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
