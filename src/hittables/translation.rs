use crate::data::interval::Interval;
use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::{HitRecord, Hittable};
use nalgebra::Vector3;

#[derive(Clone)]
pub struct Translation {
    pub hittable: Box<Hittable>,
    pub offset: Vector3<f64>,
}

impl Translation {
    pub fn bounding_box(&self, time0: f64, time1: f64) -> Option<AxisAlignedBoundingBox> {
        let orig_bounding_box = self.hittable.bounding_box(time0, time1);

        match orig_bounding_box {
            None => orig_bounding_box,

            Some(aabb) => Some(AxisAlignedBoundingBox {
                minimum: Point64(aabb.minimum.0 + self.offset),
                maximum: Point64(aabb.maximum.0 + self.offset),
            }),
        }
    }

    pub fn is_hit_by(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let moved_ray = Ray {
            origin: Point64(ray.origin.0 - self.offset),
            direction: ray.direction,
            exposure_time: ray.exposure_time,
        };

        // Book 2, section 8.1: hit in object space, then move the hit point back. The normal,
        // front_face, t and uv are unaffected by translation, so keep the child's values
        // (rebuilding with HitRecord::new would re-derive front_face from an already-flipped
        // normal and always get `true`).
        self.hittable
            .is_hit_by(&moved_ray, ray_t)
            .map(|hr| HitRecord {
                location: Point64(hr.location.0 + self.offset),
                ..hr
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::hittables::sphere::Sphere;
    use crate::materials::Material;
    use crate::materials::dielectric::Dielectric;
    use approx::assert_abs_diff_eq;

    /// Unit sphere at the origin, translated to (10, 0, 0).
    fn moved_sphere() -> Translation {
        Translation {
            hittable: Box::new(Hittable::Sphere(Sphere {
                center: Point64::new(0., 0., 0.),
                radius: 1.,
                material: Material::Dielectric(Dielectric {
                    index_of_refraction: 1.5,
                }),
            })),
            offset: Vector3::new(10., 0., 0.),
        }
    }

    const RANGE: Interval = Interval::new(0.001, f64::INFINITY);

    #[test]
    fn hit_location_is_in_world_space() {
        let r = Ray {
            origin: Point64::new(10., 0., 5.),
            direction: Point64::new(0., 0., -1.),
            exposure_time: 0.,
        };
        let hr = moved_sphere().is_hit_by(&r, RANGE).unwrap();
        assert_abs_diff_eq!(hr.value, 4., epsilon = 1e-12);
        assert_abs_diff_eq!(hr.location.0, Point64::new(10., 0., 1.).0, epsilon = 1e-12);
        assert_abs_diff_eq!(
            hr.location.0,
            r.point_at_parameter(hr.value).0,
            epsilon = 1e-12
        );
        assert_abs_diff_eq!(hr.normal.0, Point64::new(0., 0., 1.).0, epsilon = 1e-12);
        assert!(hr.front_face);
    }

    #[test]
    fn hit_from_inside_is_a_back_face() {
        let r = Ray {
            origin: Point64::new(10., 0., 0.),
            direction: Point64::new(0., 0., -1.),
            exposure_time: 0.,
        };
        let hr = moved_sphere().is_hit_by(&r, RANGE).unwrap();
        assert_abs_diff_eq!(hr.location.0, Point64::new(10., 0., -1.).0, epsilon = 1e-12);
        assert!(!hr.front_face);
        // Normal still opposes the ray.
        assert_abs_diff_eq!(hr.normal.0, Point64::new(0., 0., 1.).0, epsilon = 1e-12);
    }

    #[test]
    fn bounding_box_is_offset() {
        let b = moved_sphere().bounding_box(0., 1.).unwrap();
        assert_abs_diff_eq!(b.minimum.0, Point64::new(9., -1., -1.).0);
        assert_abs_diff_eq!(b.maximum.0, Point64::new(11., 1., 1.).0);
    }
}
