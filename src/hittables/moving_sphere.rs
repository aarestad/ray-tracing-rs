use crate::data::interval::Interval;
use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::HitRecord;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::sphere::get_sphere_uv;
use crate::materials::Material;
use nalgebra::Vector3;

#[derive(Clone)]
pub struct MovingSphere {
    pub center0: Point64,
    pub center1: Point64,
    pub radius: f64,
    pub material: Material,
    pub time0: f64,
    pub time1: f64,
}

impl MovingSphere {
    pub fn center_at(&self, time: f64) -> Point64 {
        self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }

    pub fn bounding_box(&self, time0: f64, time1: f64) -> Option<AxisAlignedBoundingBox> {
        let half_box_side = Vector3::new(self.radius, self.radius, self.radius);
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

    pub fn is_hit_by(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let ray_origin_to_center = ray.origin - self.center_at(ray.exposure_time);
        let a = ray.direction.0.dot(&ray.direction.0);
        let half_b = ray_origin_to_center.0.dot(&ray.direction.0);
        let c = ray_origin_to_center.0.dot(&ray_origin_to_center.0) - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant >= 0. {
            let sqrt_discriminant = discriminant.sqrt();
            let root_one = (-half_b - sqrt_discriminant) / a;
            let root_two = (-half_b + sqrt_discriminant) / a;
            let root_one_in_range = ray_t.surrounds(root_one);
            let root_two_in_range = ray_t.surrounds(root_two);

            if root_one_in_range || root_two_in_range {
                let root = if root_one_in_range {
                    root_one
                } else {
                    root_two
                };

                let location = ray.point_at_parameter(root);
                let outward_normal = (location - self.center_at(ray.exposure_time)) / self.radius;

                Some(HitRecord::new(
                    root,
                    ray,
                    outward_normal,
                    self.material.clone(),
                    get_sphere_uv(outward_normal),
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::color64::Color64;
    use crate::materials::diffuse_light::DiffuseLight;
    use approx::assert_abs_diff_eq;

    fn sliding_sphere() -> MovingSphere {
        MovingSphere {
            center0: Point64::new(0., 0., -5.),
            center1: Point64::new(2., 0., -5.),
            radius: 0.5,
            material: Material::DiffuseLight(DiffuseLight::new(Color64::new(1., 1., 1.))),
            time0: 0.,
            time1: 1.,
        }
    }

    #[test]
    fn center_interpolates_over_time() {
        let s = sliding_sphere();
        assert_abs_diff_eq!(s.center_at(0.).0, Point64::new(0., 0., -5.).0);
        assert_abs_diff_eq!(s.center_at(0.5).0, Point64::new(1., 0., -5.).0);
        assert_abs_diff_eq!(s.center_at(1.).0, Point64::new(2., 0., -5.).0);
    }

    #[test]
    fn hit_depends_on_ray_time() {
        let s = sliding_sphere();
        let ray_at = |time| Ray {
            origin: Point64::new(2., 0., 0.),
            direction: Point64::new(0., 0., -1.),
            exposure_time: time,
        };
        let range = Interval::new(0.001, f64::INFINITY);
        assert!(s.is_hit_by(&ray_at(0.), range).is_none());
        let hit = s.is_hit_by(&ray_at(1.), range).unwrap();
        assert_abs_diff_eq!(hit.value, 4.5, epsilon = 1e-12);
        assert_abs_diff_eq!(hit.normal.0, Point64::new(0., 0., 1.).0, epsilon = 1e-12);
    }

    #[test]
    fn bounding_box_covers_whole_motion() {
        let b = sliding_sphere().bounding_box(0., 1.).unwrap();
        assert_abs_diff_eq!(b.minimum.0, Point64::new(-0.5, -0.5, -5.5).0);
        assert_abs_diff_eq!(b.maximum.0, Point64::new(2.5, 0.5, -4.5).0);
    }
}
