use crate::data::ray::Ray;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::bvh_comparators::BOX_COMPARATORS;
use crate::hittables::{HitRecord, Hittable};
use rand::prelude::IndexedRandom;
use std::sync::Arc;

pub struct BoundedVolumeHierarchy {
    left_child: Arc<Hittable>,
    right_child: Arc<Hittable>,
    left_bounds: AxisAlignedBoundingBox,
    right_bounds: AxisAlignedBoundingBox,
    bounding_box: AxisAlignedBoundingBox,
}

impl BoundedVolumeHierarchy {
    pub fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AxisAlignedBoundingBox> {
        Some(self.bounding_box)
    }

    pub fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord> {
        if !self.bounding_box.is_hit_by(ray, min_value, max_value) {
            return None;
        }

        if Arc::ptr_eq(&self.left_child, &self.right_child) {
            return self.left_child.is_hit_by(ray, min_value, max_value);
        }

        let hit_left_box = self.left_bounds.hit_interval(ray, min_value, max_value);
        let hit_right_box = self.right_bounds.hit_interval(ray, min_value, max_value);

        match (hit_left_box, hit_right_box) {
            (None, None) => None,
            (Some(_), None) => self.left_child.is_hit_by(ray, min_value, max_value),
            (None, Some(_)) => self.right_child.is_hit_by(ray, min_value, max_value),
            (Some((la, _)), Some((rb, _))) => {
                let (first, second) = if la <= rb {
                    (&self.left_child, &self.right_child)
                } else {
                    (&self.right_child, &self.left_child)
                };

                let hit_first = first.is_hit_by(ray, min_value, max_value);
                let t_max2 = hit_first.as_ref().map_or(max_value, |hr| hr.value);
                let hit_second = second.is_hit_by(ray, min_value, t_max2);
                hit_second.or(hit_first)
            }
        }
    }

    pub fn create_bvh_arc(
        objects: &mut [Arc<Hittable>],
        time0: f64,
        time1: f64,
    ) -> Arc<Hittable> {
        let comparator = BOX_COMPARATORS.choose(&mut rand::rng()).unwrap();

        let left_child: Arc<Hittable>;
        let right_child: Arc<Hittable>;

        match objects.len() {
            0 => panic!("empty list of hittables passed to BoundedVolumeHierarchy::new"),
            1 => {
                left_child = objects[0].clone();
                right_child = objects[0].clone();
            }
            2 => {
                let o1 = &objects[0].clone();
                let o2 = &objects[1].clone();

                match comparator(o1, o2) {
                    std::cmp::Ordering::Less => {
                        left_child = objects[0].clone();
                        right_child = objects[1].clone();
                    }
                    _ => {
                        left_child = objects[1].clone();
                        right_child = objects[0].clone();
                    }
                }
            }
            _ => {
                objects.sort_by(comparator);
                let mid = objects.len() / 2;
                left_child = BoundedVolumeHierarchy::create_bvh_arc(
                    &mut objects[0..mid].to_vec(),
                    time0,
                    time1,
                )
                .clone();
                right_child = BoundedVolumeHierarchy::create_bvh_arc(
                    &mut objects[mid..].to_vec(),
                    time0,
                    time1,
                )
                .clone();
            }
        }

        let box_left = left_child
            .bounding_box(time0, time1)
            .expect("No bounding box in bvh_node constructor for hittable");
        let box_right = right_child
            .bounding_box(time0, time1)
            .expect("No bounding box in bvh_node constructor for hittable");

        Arc::new(Hittable::Bvh(BoundedVolumeHierarchy {
            left_child,
            right_child,
            left_bounds: box_left,
            right_bounds: box_right,
            bounding_box: box_left.surrounding_box_with(&box_right),
        }))
    }
}
