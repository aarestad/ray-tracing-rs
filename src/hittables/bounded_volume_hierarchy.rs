use crate::data::ray::Ray;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::bvh_comparators::BOX_COMPARATORS;
use crate::hittables::{HitRecord, Hittable};
use rand::Rng;
use std::sync::Arc;

pub struct BoundedVolumeHierarchy {
    left_child: Arc<dyn Hittable>,
    right_child: Arc<dyn Hittable>,
    bounding_box: AxisAlignedBoundingBox,
}

impl Hittable for BoundedVolumeHierarchy {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AxisAlignedBoundingBox> {
        Some(self.bounding_box)
    }

    fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord> {
        if !self.bounding_box.is_hit_by(ray, min_value, max_value) {
            return None;
        }

        let hit_left = self.left_child.is_hit_by(ray, min_value, max_value);

        let hit_right = self.right_child.is_hit_by(
            ray,
            min_value,
            hit_left.as_ref().map_or(max_value, |hr| hr.value),
        );

        return hit_left.or(hit_right);
    }
}

impl BoundedVolumeHierarchy {
    pub fn new(objects: &mut Vec<Box<dyn Hittable>>, time0: f64, time1: f64) -> Box<dyn Hittable> {
        let comparator = BOX_COMPARATORS[rand::thread_rng().gen_range(0..3)];

        let left_child: &Box<dyn Hittable>;
        let right_child: &Box<dyn Hittable>;

        match objects.len() {
            0 => panic!("empty list of hittables passed to BoundedVolumeHierarchy::new"),
            1 => {
                left_child = &objects[0];
                right_child = &objects[0];
            },
            2 => {
                let o1 = &objects[0];
                let o2 = &objects[1];

                match comparator(o1, o2) {
                    Less => {
                        left_child = &objects[0];
                        right_child = &objects[1];
                    },
                    _ => {
                        left_child = &objects[1];
                        right_child = &objects[0];
                    }
                }
            },
            _ => {
                objects.sort_by(comparator);
                let mid = objects.len() / 2;
                left_child = &BoundedVolumeHierarchy::new(&mut objects[0..mid].to_vec(), time0, time1);
                right_child = &BoundedVolumeHierarchy::new(&mut objects[mid..].to_vec(), time0, time1);
            }
        }

        let box_left = left_child.bounding_box(time0, time1);
        let box_right = right_child.bounding_box(time0, time1);

        if box_left.is_none() || box_right.is_none() {
            panic!("No bounding box in bvh_node constructor for hittable");
        }

        Box::from(BoundedVolumeHierarchy {
            left_child: Arc::new(left_child.into()),
            right_child: Arc::new(right_child.into()),
            bounding_box: box_left
                .unwrap()
                .surrounding_box_with(box_right.as_ref().unwrap()),
        })
    }
}
