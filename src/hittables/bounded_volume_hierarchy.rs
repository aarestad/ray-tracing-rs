use crate::data::interval::Interval;
use crate::data::ray::Ray;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::bvh_comparators::BOX_COMPARATORS;
use crate::hittables::{HitRecord, Hittable};

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
        if !self.bounding_box.is_hit_by(ray, ray_t) {
            return None;
        }

        let hit_left_box = self.left_bounds.hit_interval(ray, ray_t);
        let hit_right_box = self.right_bounds.hit_interval(ray, ray_t);

        match (hit_left_box, hit_right_box) {
            (None, None) => None,
            (Some(_), None) => self.left_child.is_hit_by(ray, ray_t),
            (None, Some(_)) => self.right_child.is_hit_by(ray, ray_t),
            (Some(left), Some(right)) => {
                // Visit the nearer child first so its hit can shrink the search for the other.
                let (first, second) = if left.min <= right.min {
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
        // Book 2, section 3.10: split along the longest axis of the span's bounding box.
        let span_box = objects
            .iter()
            .map(|o| {
                o.bounding_box(time0, time1)
                    .expect("No bounding box in bvh_node constructor for hittable")
            })
            .reduce(|a, b| a.surrounding_box_with(&b));
        let comparator = BOX_COMPARATORS[span_box.map_or(0, |b| b.longest_axis())];

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::color64::Color64;
    use crate::data::point64::Point64;
    use crate::data::vector3::rand_range;
    use crate::hittables::hittable_vec::HittableVec;
    use crate::hittables::moving_sphere::MovingSphere;
    use crate::hittables::quad::Quad;
    use crate::hittables::sphere::Sphere;
    use crate::materials::Material;
    use crate::materials::diffuse_light::DiffuseLight;

    fn random_scene(n: usize) -> Vec<Hittable> {
        let mat = Material::DiffuseLight(DiffuseLight::new(Color64::new(1., 1., 1.)));
        (0..n)
            .map(|i| match i % 3 {
                0 => Hittable::Sphere(Sphere {
                    center: Point64(rand_range(-10., 10.)),
                    radius: 0.5,
                    material: mat.clone(),
                }),
                1 => {
                    let c = Point64(rand_range(-10., 10.));
                    Hittable::MovingSphere(MovingSphere {
                        center0: c,
                        center1: Point64(c.0 + rand_range(-1., 1.)),
                        radius: 0.3,
                        material: mat.clone(),
                        time0: 0.,
                        time1: 1.,
                    })
                }
                // Axis-aligned quads exercise the padded zero-thickness bounding boxes.
                _ => Hittable::Quad(Quad::new(
                    Point64(rand_range(-10., 10.)),
                    Point64::new(1., 0., 0.),
                    Point64::new(0., 0., 1.),
                    mat.clone(),
                )),
            })
            .collect()
    }

    #[test]
    fn bvh_finds_same_closest_hits_as_linear_list() {
        let mut objects = random_scene(300);
        let list = HittableVec {
            hittables: objects.clone(),
        };
        let bvh = BoundedVolumeHierarchy::create_bvh(&mut objects, 0., 1.);
        let range = Interval::new(0.001, f64::INFINITY);

        let mut hits = 0;
        for _ in 0..5_000 {
            let ray = Ray {
                origin: Point64(rand_range(-12., 12.)),
                direction: Point64(rand_range(-1., 1.)),
                exposure_time: rand::random(),
            };
            let expected = list.is_hit_by(&ray, range).map(|h| h.value);
            let actual = bvh.is_hit_by(&ray, range).map(|h| h.value);
            assert_eq!(expected, actual);
            hits += expected.is_some() as usize;
        }
        assert!(
            hits > 100,
            "test rays should hit something reasonably often ({hits})"
        );
    }

    #[test]
    fn single_object_bvh() {
        let mut objects = random_scene(1);
        let bvh = BoundedVolumeHierarchy::create_bvh(&mut objects, 0., 1.);
        assert!(bvh.bounding_box(0., 1.).is_some());
    }
}
