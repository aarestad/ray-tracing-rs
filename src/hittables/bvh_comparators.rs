use crate::hittables::Hittable;
use std::cmp::Ordering;
use std::sync::Arc;

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
    let box_a = a.bounding_box(0., 0.).expect("bad bounding box");
    let box_b = b.bounding_box(0., 0.).expect("bad bounding box");

    let box_a_element = box_a.minimum.0[axis];
    let box_b_element = box_b.minimum.0[axis];

    box_a_element
        .partial_cmp(&box_b_element)
        .expect("got an NaN as a box dimension value")
}

type BoxComparator = fn(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> Ordering;

fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}

pub(crate) const BOX_COMPARATORS: [BoxComparator; 3] =
    [box_x_compare, box_y_compare, box_z_compare];
