use crate::data::ray::Ray;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::{HitRecord, Hittable};

pub struct HittableVec {
    pub hittables: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl Hittable for HittableVec {
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AxisAlignedBoundingBox> {
        if self.hittables.is_empty() {
            return None;
        };

        // TODO this looks janky....
        let mut out = self.hittables[0].bounding_box(time0, time1);

        if out.is_none() {
            return None;
        }

        for hittable in self.hittables[1..].iter() {
            let next_box = hittable.bounding_box(time0, time1);

            if next_box.is_none() {
                return None;
            }

            out = Some(out.unwrap().surrounding_box_with(&next_box.unwrap()));
        }

        out
    }

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
