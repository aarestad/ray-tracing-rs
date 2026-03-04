use std::f64::consts::{PI, TAU};
use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::data::vector3::Vector;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::bvh_comparators::BOX_COMPARATORS;
use crate::materials::Materials;
use nalgebra::Vector3;
use rand::prelude::IndexedRandom;
use crate::util::EPSILON;

mod axis_aligned_bounding_box;
mod bvh_comparators;

#[derive(Clone)]
pub struct HitRecord {
    pub value: f64,
    pub u: f64,
    pub v: f64,
    pub location: Point64,
    pub normal: Point64,
    pub front_face: bool,
    pub material: Materials,
}

impl HitRecord {
    pub fn new(
        value: f64,
        ray: &Ray,
        outward_normal: Point64,
        material: Materials,
        uv: (f64, f64),
    ) -> HitRecord {
        let front_face = ray.direction.0.dot(&outward_normal.0) < 0.;

        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            value,
            u: uv.0,
            v: uv.1,
            location: ray.point_at_parameter(value),
            normal,
            front_face,
            material,
        }
    }
}

#[derive(Clone)]
pub enum AxisAlignment {
    X,
    Y,
    Z,
}

impl AxisAlignment {
    pub const fn to_usize(&self) -> usize {
        match self {
            AxisAlignment::X => 0,
            AxisAlignment::Y => 1,
            AxisAlignment::Z => 2,
        }
    }
}

fn get_sphere_uv(p: Point64) -> (f64, f64) {
    let theta = (-p.y()).acos();
    let phi = (-p.z()).atan2(p.x()) + PI;

    (phi / TAU, theta / PI)
}

#[derive(Clone)]
pub enum Hittables {
    // min and max are a 2-dimensional pair depending on axis alignment
    // X -> (y, z)
    // Y -> (x, z)
    // Z -> (x, y)
    AxisAlignedRect(
        Materials,
        (f64, f64), /* min */
        (f64, f64), /* max */
        f64,
        AxisAlignment,
    ),
    BoundedVolumeHierarchy(Box<Hittables>, Box<Hittables>, AxisAlignedBoundingBox),
    Cuboid(
        Point64,
        Point64,
        Box<Hittables>, /* should be a HittableVec */
    ),
    HittableVec(Vec<Box<Hittables>>),
    MovingSphere(Point64, Point64, f64, Materials, f64, f64),
    Sphere(Point64, f64, Materials),
    Translation(Box<Hittables>, Vector3<f64>),
    Triangle(Point64, Point64, Point64, Vector, Vector, Vector, Materials),
}

impl Hittables {
    pub fn new_bvh(objects: Vec<Self>, time0: f64, time1: f64) -> Self {
        let comparator = BOX_COMPARATORS.choose(&mut rand::rng()).unwrap();

        let left_child: Hittables;
        let right_child: Hittables;

        match objects.len() {
            0 => panic!("empty list of hittables passed to BoundedVolumeHierarchy::new"),
            1 => {
                left_child = objects[0].clone();
                right_child = objects[0].clone();
            }
            2 => {
                let o1 = &objects[0];
                let o2 = &objects[1];

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
                let mut objects = objects;
                objects.sort_by(comparator);
                let mid = objects.len() / 2;
                left_child = Self::new_bvh(objects.split_at(mid).0.to_vec(), time0, time1);
                right_child = Self::new_bvh(objects.split_at(mid).1.to_vec(), time0, time1);
            }
        }

        let box_left = left_child
            .bounding_box(time0, time1)
            .expect("No bounding box in bvh_node constructor for hittable");
        let box_right = right_child
            .bounding_box(time0, time1)
            .expect("No bounding box in bvh_node constructor for hittable");

        Hittables::BoundedVolumeHierarchy(
            Box::from(left_child),
            Box::from(right_child),
            box_left.surrounding_box_with(&box_right),
        )
    }

    pub fn new_cuboid(cuboid_min: Point64, cuboid_max: Point64, material: Materials) -> Self {
        Hittables::Cuboid(
            cuboid_min,
            cuboid_max,
            Box::from(Hittables::HittableVec(vec![
                Box::from(Hittables::AxisAlignedRect(
                    material.clone(),
                    (cuboid_min.y(), cuboid_min.z()),
                    (cuboid_max.y(), cuboid_max.z()),
                    cuboid_max.x(),
                    AxisAlignment::X,
                )),
                Box::from(Hittables::AxisAlignedRect(
                    material.clone(),
                    (cuboid_min.y(), cuboid_min.z()),
                    (cuboid_max.y(), cuboid_max.z()),
                    cuboid_min.x(),
                    AxisAlignment::X,
                )),
                Box::from(Hittables::AxisAlignedRect(
                    material.clone(),
                    (cuboid_min.x(), cuboid_min.z()),
                    (cuboid_max.x(), cuboid_max.z()),
                    cuboid_max.y(),
                    AxisAlignment::Y,
                )),
                Box::from(Hittables::AxisAlignedRect(
                    material.clone(),
                    (cuboid_min.x(), cuboid_min.z()),
                    (cuboid_max.x(), cuboid_max.z()),
                    cuboid_min.y(),
                    AxisAlignment::Y,
                )),
                Box::from(Hittables::AxisAlignedRect(
                    material.clone(),
                    (cuboid_min.x(), cuboid_min.y()),
                    (cuboid_max.x(), cuboid_max.y()),
                    cuboid_max.z(),
                    AxisAlignment::Z,
                )),
                Box::from(Hittables::AxisAlignedRect(
                    material,
                    (cuboid_min.x(), cuboid_min.y()),
                    (cuboid_max.x(), cuboid_max.y()),
                    cuboid_min.z(),
                    AxisAlignment::Z,
                )),
            ])),
        )
    }

    fn center_at(&self, time: f64) -> Point64 {
        match self {
            Hittables::MovingSphere(center0, center1, _, _, time0, time1) => {
                *center0
                    + (*center1 - *center0) * ((time - *time0) / (*time1 - *time0))
            },
            _ => Point64::default()
        }

    }

    pub fn bounding_box(&self, time0: f64, time1: f64) -> Option<AxisAlignedBoundingBox> {
        match self {
            Hittables::AxisAlignedRect(_, min, max, axis_value, axis_alignment) => {
                match axis_alignment {
                    AxisAlignment::X => Some(AxisAlignedBoundingBox {
                        minimum: Point64::new(axis_value - 0.0001, min.0, min.1),
                        maximum: Point64::new(axis_value + 0.0001, max.0, max.1),
                    }),
                    AxisAlignment::Y => Some(AxisAlignedBoundingBox {
                        minimum: Point64::new(min.0, axis_value - 0.0001, min.1),
                        maximum: Point64::new(max.0, axis_value + 0.0001, max.1),
                    }),
                    AxisAlignment::Z => Some(AxisAlignedBoundingBox {
                        minimum: Point64::new(min.0, min.1, axis_value - 0.0001),
                        maximum: Point64::new(max.0, max.1, axis_value + 0.0001),
                    }),
                }
            }
            Hittables::BoundedVolumeHierarchy(_, _, bounding_box) => Some(*bounding_box),
            Hittables::Cuboid(cuboid_min, cuboid_max, _) => {
                Some(AxisAlignedBoundingBox {
                    minimum: *cuboid_min,
                    maximum: *cuboid_max,
                })
            }
            Hittables::HittableVec(hittables) => {
                if hittables.is_empty() {
                    return None;
                };

                hittables
                    .iter()
                    .fold(hittables[0].bounding_box(time0, time1), |acc, hittable| {
                        Some(
                            acc?.surrounding_box_with(
                                &hittable.as_ref().bounding_box(time0, time1)?,
                            ),
                        )
                    })
            }
            Hittables::MovingSphere(_, _, radius, _, _, _) => {
                let half_box_side = Vector3::new(*radius, *radius, *radius);
                let center0 = self.center_at(time0);
                let center1 = self.center_at(time1);

                let box0 = AxisAlignedBoundingBox {
                    minimum: Point64(center0.0 - half_box_side),
                    maximum: Point64(center0.0 + half_box_side),
                };

                let box1 = AxisAlignedBoundingBox {
                    minimum: Point64(center1.0 - half_box_side),
                    maximum: Point64(center1.0 + half_box_side),
                };

                Some(box0.surrounding_box_with(&box1))
            }
            Hittables::Sphere(center, radius, _) => {Some(AxisAlignedBoundingBox {
                minimum: Point64(center.0 - Vector3::new(*radius, *radius, *radius)),
                maximum: Point64(center.0 + Vector3::new(*radius, *radius, *radius)),
            })}
            Hittables::Translation(hittable, offset) => {
                let orig_bounding_box = hittable.bounding_box(time0, time1);

                match orig_bounding_box {
                    None => orig_bounding_box,

                    Some(aabb) => Some(AxisAlignedBoundingBox {
                        minimum: Point64(aabb.minimum.0 + offset),
                        maximum: Point64(aabb.maximum.0 + offset),
                    }),
                }
            }
            Hittables::Triangle(p1, p2, p3, _, _, _, _) => {
                let min_x = p1.x().min(p2.x().min(p3.x()));
                let min_y = p1.y().min(p2.y().min(p3.y()));
                let min_z = p1.z().min(p2.z().min(p3.z()));

                let max_x = p1.x().max(p2.x().max(p3.x()));
                let max_y = p1.y().max(p2.y().max(p3.y()));
                let max_z = p1.z().max(p2.z().max(p3.z()));

                Some(AxisAlignedBoundingBox {
                    minimum: Point64::new(min_x, min_y, min_z),
                    maximum: Point64::new(max_x, max_y, max_z),
                })
            }
        }
    }

    pub fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord> {
        match self {
            Hittables::AxisAlignedRect(material, min, max, axis_value, axis_alignment) => {
                let t = (axis_value - ray.origin.0[axis_alignment.to_usize()])
                    / ray.direction.0[axis_alignment.to_usize()];

                if !(min_value..max_value).contains(&t) {
                    return None;
                }

                let (c1, c2): (f64, f64) = match axis_alignment {
                    AxisAlignment::X => (
                        ray.origin.y() + t * ray.direction.y(),
                        ray.origin.z() + t * ray.direction.z(),
                    ),
                    AxisAlignment::Y => (
                        ray.origin.x() + t * ray.direction.x(),
                        ray.origin.z() + t * ray.direction.z(),
                    ),
                    AxisAlignment::Z => (
                        ray.origin.x() + t * ray.direction.x(),
                        ray.origin.y() + t * ray.direction.y(),
                    ),
                };

                if !(min.0..max.0).contains(&c1) || !(min.1..max.1).contains(&c2) {
                    return None;
                }

                Some(HitRecord::new(
                    t,
                    ray,
                    Point64::new(0., 0., 1.),
                    material.clone(),
                    (
                        (c1 - min.0) / (max.0 - min.0),
                        (c2 - min.1) / (max.1 - min.1),
                    ),
                ))
            }
            Hittables::BoundedVolumeHierarchy(left_child, right_child, bounding_box) => {
                if !bounding_box.is_hit_by(ray, min_value, max_value) {
                    return None;
                }

                let hit_left = left_child.is_hit_by(ray, min_value, max_value);

                let hit_right = right_child.is_hit_by(
                    ray,
                    min_value,
                    hit_left.as_ref().map_or(max_value, |hr| hr.value),
                );

                hit_right.or(hit_left)
            }
            Hittables::Cuboid(_, _, sides) => {
                sides.is_hit_by(ray, min_value, max_value)
            }
            Hittables::HittableVec(hittables) => {
                let mut winner: Option<HitRecord> = None;

                for hittable in hittables {
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
            Hittables::MovingSphere(_, _, radius, material, _, _) => {
                let ray_origin_to_center = ray.origin - self.center_at(ray.exposure_time);
                let a = ray.direction.0.dot(&ray.direction.0);
                let half_b = ray_origin_to_center.0.dot(&ray.direction.0);
                let c = ray_origin_to_center.0.dot(&ray_origin_to_center.0) - radius.powi(2);
                let discriminant = half_b.powi(2) - a * c;
                let sqrt_discriminant = discriminant.sqrt();
                let root_one = (-half_b - sqrt_discriminant) / a;
                let root_two = (-half_b + sqrt_discriminant) / a;

                if discriminant >= 0. {
                    let root_one_in_range = min_value < root_one && root_one < max_value;
                    let root_two_in_range = min_value < root_two && root_two < max_value;

                    if root_one_in_range || root_two_in_range {
                        let root = if root_one_in_range {
                            root_one
                        } else {
                            root_two
                        };

                        let location = ray.point_at_parameter(root);
                        let outward_normal = (location - self.center_at(ray.exposure_time)) / *radius;

                        Some(HitRecord::new(
                            root,
                            ray,
                            outward_normal,
                            material.clone(),
                            get_sphere_uv(outward_normal),
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Hittables::Sphere(center, radius, material) => {
                let ray_origin_to_center: Point64 = ray.origin - *center;
                let a = ray.direction.0.dot(&ray.direction.0);
                let half_b = ray_origin_to_center.0.dot(&ray.direction.0);
                let c = ray_origin_to_center.0.dot(&ray_origin_to_center.0) - radius.powi(2);
                let discriminant = half_b.powi(2) - a * c;
                let sqrt_discriminant = discriminant.sqrt();
                let root_one = (-half_b - sqrt_discriminant) / a;
                let root_two = (-half_b + sqrt_discriminant) / a;

                if discriminant >= 0. {
                    let root_one_in_range = min_value < root_one && root_one < max_value;
                    let root_two_in_range = min_value < root_two && root_two < max_value;

                    if root_one_in_range || root_two_in_range {
                        let root = if root_one_in_range {
                            root_one
                        } else {
                            root_two
                        };

                        let location = ray.point_at_parameter(root);
                        let outward_normal = (location - *center) / *radius;

                        return Some(HitRecord::new(
                            root,
                            ray,
                            outward_normal,
                            material.clone(),
                            get_sphere_uv(outward_normal),
                        ));
                    }
                }

                None
            }
            Hittables::Translation(hittable, offset) => {
                let moved_ray = Ray {
                    origin: Point64(ray.origin.0 - offset),
                    direction: ray.direction,
                    exposure_time: ray.exposure_time,
                };

                let opt_hit_record = hittable.is_hit_by(&moved_ray, min_value, max_value);

                match opt_hit_record {
                    Some(hr) => Some(HitRecord::new(
                        hr.value,
                        &moved_ray,
                        hr.normal,
                        hr.material,
                        (hr.u, hr.v),
                    )),
                    None => None,
                }
            }
            Hittables::Triangle(p1, _, _, e1, e2, normal, material) => {
                let cross_e2 = ray.direction.0.cross(&e2);
                let determinant = e1.dot(&cross_e2);

                if determinant.abs() < EPSILON {
                    return None;
                }

                let f = 1.0 / determinant;
                let p1_to_origin = ray.origin.0 - p1.0;
                let u = f * p1_to_origin.dot(&cross_e2);

                if !(0.0..=1.0).contains(&u) {
                    return None;
                }

                let origin_cross_e1 = p1_to_origin.cross(&e1);
                let v = f * ray.direction.0.dot(&origin_cross_e1);

                if v < 0.0 || (u + v) > 1.0 {
                    return None;
                }

                Some(HitRecord::new(
                    f * e2.dot(&origin_cross_e1),
                    ray,
                    Point64(*normal),
                    material.clone(),
                    (u, v),
                ))
            }
        }
    }
}
