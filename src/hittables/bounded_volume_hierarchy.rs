use crate::data::interval::Interval;
use crate::data::ray::Ray;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::bvh_comparators::BOX_COMPARATORS;
use crate::hittables::{HitRecord, Hittable};
use rand::prelude::IndexedRandom;

#[derive(Clone)]
pub struct BoundedVolumeHierarchy {
    left_child: Box<Hittable>,
    right_child: Box<Hittable>,
    left_bounds: AxisAlignedBoundingBox,
    right_bounds: AxisAlignedBoundingBox,
    bounding_box: AxisAlignedBoundingBox,
}

impl BoundedVolumeHierarchy {
    pub fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AxisAlignedBoundingBox> {
        Some(self.bounding_box)
    }

    pub fn is_hit_by(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if !self.bounding_box.is_hit_by(ray, ray_t.min, ray_t.max) {
            return None;
        }

        let hit_left_box = self.left_bounds.hit_interval(ray, ray_t.min, ray_t.max);
        let hit_right_box = self.right_bounds.hit_interval(ray, ray_t.min, ray_t.max);

        match (hit_left_box, hit_right_box) {
            (None, None) => None,
            (Some(_), None) => self.left_child.is_hit_by(ray, ray_t),
            (None, Some(_)) => self.right_child.is_hit_by(ray, ray_t),
            (Some((la, _)), Some((rb, _))) => {
                let (first, second) = if la <= rb {
                    (&self.left_child, &self.right_child)
                } else {
                    (&self.right_child, &self.left_child)
                };

                let hit_first = first.is_hit_by(ray, ray_t);
                let t_max2 = hit_first.as_ref().map_or(ray_t.max, |hr| hr.value);
                let hit_second = second.is_hit_by(ray, ray_t.with_max(t_max2));
                hit_second.or(hit_first)
            }
        }
    }

    pub fn create_bvh(objects: &mut [Hittable], time0: f64, time1: f64) -> Hittable {
        let comparator = BOX_COMPARATORS.choose(&mut rand::rng()).unwrap();

        let left_child: Box<Hittable>;
        let right_child: Box<Hittable>;

        match objects.len() {
            0 => panic!("empty list of hittables passed to BoundedVolumeHierarchy::new"),
            1 => {
                left_child = Box::new(objects[0].clone());
                right_child = Box::new(objects[0].clone());
            }
            2 => {
                let o1 = &objects[0];
                let o2 = &objects[1];

                match comparator(o1, o2) {
                    std::cmp::Ordering::Less => {
                        left_child = Box::new(objects[0].clone());
                        right_child = Box::new(objects[1].clone());
                    }
                    _ => {
                        left_child = Box::new(objects[1].clone());
                        right_child = Box::new(objects[0].clone());
                    }
                }
            }
            _ => {
                objects.sort_by(comparator);
                let mid = objects.len() / 2;
                left_child = Box::new(BoundedVolumeHierarchy::create_bvh(
                    &mut objects[0..mid].to_vec(),
                    time0,
                    time1,
                ));
                right_child = Box::new(BoundedVolumeHierarchy::create_bvh(
                    &mut objects[mid..].to_vec(),
                    time0,
                    time1,
                ));
            }
        }

        let box_left = left_child
            .bounding_box(time0, time1)
            .expect("No bounding box in bvh_node constructor for hittable");
        let box_right = right_child
            .bounding_box(time0, time1)
            .expect("No bounding box in bvh_node constructor for hittable");

        Hittable::Bvh(BoundedVolumeHierarchy {
            left_child,
            right_child,
            left_bounds: box_left,
            right_bounds: box_right,
            bounding_box: box_left.surrounding_box_with(&box_right),
        })
    }
}
