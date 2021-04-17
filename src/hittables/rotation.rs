use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::{Hittable, HitRecord};
use crate::data::ray::Ray;
use crate::data::point64::Point64;
use nalgebra::Vector3;

pub struct Rotation {
    hittable: Box<dyn Hittable>,
    sin_cos_theta: (f64, f64),
    bounding_box: AxisAlignedBoundingBox,
}

// TODO implement x and z rotations
impl Rotation {
    pub fn new_y(hittable: Box<dyn Hittable>, theta_deg: f64) -> Rotation {
        let theta = theta_deg.to_radians();
        let (sin_theta, cos_theta) = (theta.sin(), theta.cos());
        let orig_bounding_box
            = hittable.bounding_box(0., 1.)
            .expect("hittable has no bounding box");

        let mut min = Vector3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vector3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        (0..2).for_each(|i| {
            (0..2).for_each(|j| {
                (0..2).for_each(|k| {
                    let x = i as f64 * orig_bounding_box.maximum.x()
                        + (1. - i as f64) * orig_bounding_box.minimum.x();

                    let y = j as f64 * orig_bounding_box.maximum.y()
                        + (1. - j as f64) * orig_bounding_box.minimum.y();

                    let z = k as f64 * orig_bounding_box.maximum.z()
                        + (1. - k as f64) * orig_bounding_box.minimum.z();

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vector3::new(new_x, y, new_z);

                    (0..3).for_each(|c| {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    });
                })
            })
        });

        Rotation {
            hittable,
            sin_cos_theta: (sin_theta, cos_theta),
            bounding_box: AxisAlignedBoundingBox {
                minimum: Point64(min),
                maximum: Point64(max),
            },
        }
    }
}

impl Hittable for Rotation {
    fn bounding_box(&self, _: f64, _: f64) -> Option<AxisAlignedBoundingBox> {
        Some(self.bounding_box)
    }

    fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord> {
        let Ray { mut origin, mut direction, exposure_time } = ray;
        let (sin_theta, cos_theta) = self.sin_cos_theta;

        origin[0] = cos_theta * origin[0] - sin_theta * origin[2];
        origin[2] = cos_theta * origin[0] + sin_theta * origin[2];

        direction[0] = cos_theta * direction[0] - sin_theta * direction[2];
        direction[2] = sin_theta * direction[0] + cos_theta * direction[2];

        let rotated_ray = Ray {
            origin,
            direction,
            exposure_time: *exposure_time,
        };

        let hittable_hr
            = self.hittable.is_hit_by(&rotated_ray, min_value, max_value);

        match hittable_hr {
            None => None,
            Some(hit_record) => {
                let mut p = hit_record.location;
                let mut normal = hit_record.normal;

                p[0] = cos_theta * p[0] + sin_theta * p[2];
                p[2] = -sin_theta * p[0] + cos_theta * p[2];

                normal[0] = cos_theta * normal[0] + sin_theta * normal[2];
                normal[2] = -sin_theta * normal[0] + cos_theta * normal[2];

                Some(HitRecord::new(hit_record.value, &rotated_ray, normal,
                                    hit_record.material, (hit_record.u, hit_record.v)))
            }
        }
    }
}