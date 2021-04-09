use crate::data::ray::Ray;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::{HitRecord, Hittable};
use rand::Rng;
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BoundedVolumeHierarchy {
    left_child: Arc<dyn Hittable>,
    right_child: Arc<dyn Hittable>,
    bounding_box: AxisAlignedBoundingBox,
}

fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, axis: usize) -> Ordering {
    let box_a = a.bounding_box(0., 0.);
    let box_b = b.bounding_box(0., 0.);

    if box_a.is_none() || box_b.is_none() {
        panic!("No bounding box in bvh_node constructor for hittable");
    }

    let box_a_element = box_a.unwrap().minimum[axis];
    let box_b_element = box_b.unwrap().minimum[axis];

    if box_a_element.is_nan() || box_b_element.is_nan() {
        panic!("got an NaN as a box dimension value");
    }

    box_a_element.partial_cmp(&box_b_element).unwrap()
}

type BoxComparator = fn(&Box<dyn Hittable>, &Box<dyn Hittable>) -> Ordering;

fn box_x_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}

const BOX_COMPARATORS: [BoxComparator; 3] = [box_x_compare, box_y_compare, box_z_compare];

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
    pub fn new(objects: Vec<Box<dyn Hittable>>, time0: f64, time1: f64) -> Box<dyn Hittable> {
        let comparator = BOX_COMPARATORS[rand::thread_rng().gen_range(0..3)];

        let left_child: Box<dyn Hittable>;
        let right_child: Box<dyn Hittable>;

        // std::sort(objects.begin() + start, objects.begin() + end, comparator);
        objects.sort_by(comparator);
        let mid = objects.len() / 2;
        left_child = BoundedVolumeHierarchy::new(objects[0..mid].to_vec(), time0, time1);
        right_child = BoundedVolumeHierarchy::new(objects[mid..].to_vec(), time0, time1);

        let box_left = left_child.bounding_box(time0, time1);
        let box_right = right_child.bounding_box(time0, time1);

        if box_left.is_none() || box_right.is_none() {
            panic!("No bounding box in bvh_node constructor for hittable");
        }

        Box::from(BoundedVolumeHierarchy {
            left_child: Arc::new(left_child),
            right_child: Arc::new(right_child),
            bounding_box: box_left
                .unwrap()
                .surrounding_box_with(box_right.as_ref().unwrap()),
        })
    }
}
