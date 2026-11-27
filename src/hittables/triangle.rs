use super::{HitRecord, axis_aligned_bounding_box::AxisAlignedBoundingBox};
use crate::data::interval::Interval;
use crate::{
    data::{point64::Point64, ray::Ray, vector3::Vector},
    materials::Material,
    util::EPSILON,
};

#[derive(Clone)]
pub struct Triangle {
    p1: Point64,
    p2: Point64,
    p3: Point64,
    e1: Vector,
    e2: Vector,
    normal: Vector,
    pub material: Material,
}

impl Triangle {
    #[allow(dead_code)]
    pub fn new(p1: Point64, p2: Point64, p3: Point64, material: Material) -> Self {
        let e1 = p2.0 - p1.0;
        let e2 = p3.0 - p1.0;
        let normal = e2.cross(&e1).normalize();

        Self {
            p1,
            p2,
            p3,
            e1,
            e2,
            normal,
            material,
        }
    }

    pub fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AxisAlignedBoundingBox> {
        let min_x = self.p1.x().min(self.p2.x().min(self.p3.x()));
        let min_y = self.p1.y().min(self.p2.y().min(self.p3.y()));
        let min_z = self.p1.z().min(self.p2.z().min(self.p3.z()));

        let max_x = self.p1.x().max(self.p2.x().max(self.p3.x()));
        let max_y = self.p1.y().max(self.p2.y().max(self.p3.y()));
        let max_z = self.p1.z().max(self.p2.z().max(self.p3.z()));

        Some(AxisAlignedBoundingBox {
            minimum: Point64::new(min_x, min_y, min_z),
            maximum: Point64::new(max_x, max_y, max_z),
        })
    }

    pub fn is_hit_by(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let cross_e2 = ray.direction.0.cross(&self.e2);
        let determinant = self.e1.dot(&cross_e2);

        if determinant.abs() < EPSILON {
            return None;
        }

        let f = 1.0 / determinant;
        let p1_to_origin = ray.origin.0 - self.p1.0;
        let u = f * p1_to_origin.dot(&cross_e2);

        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let origin_cross_e1 = p1_to_origin.cross(&self.e1);
        let v = f * ray.direction.0.dot(&origin_cross_e1);

        if v < 0.0 || (u + v) > 1.0 {
            return None;
        }

        let t = f * self.e2.dot(&origin_cross_e1);
        if !ray_t.surrounds(t) {
            return None;
        }

        Some(HitRecord::new(
            t,
            ray,
            Point64(self.normal),
            self.material.clone(),
            (u, v),
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        data::{interval::Interval, point64::Point64, ray::Ray},
        materials::Material,
        materials::dielectric::Dielectric,
    };

    use super::Triangle;

    #[test]
    fn ray_parallel_to_triangle() {
        let t = Triangle::new(
            Point64::new(0., 1., 0.),
            Point64::new(-1., 0., 0.),
            Point64::new(1., 0., 0.),
            Material::Dielectric(Dielectric {
                index_of_refraction: 1.0,
            }),
        );
        let r = Ray {
            origin: Point64::new(0., -1., -2.),
            direction: Point64::new(0., 1., 0.),
            exposure_time: 1.0,
        };
        assert!(t.is_hit_by(&r, Interval::new(0.0, 100.0)).is_none());
    }

    #[test]
    fn ray_misses_p1_p3_edge() {
        let t = Triangle::new(
            Point64::new(0., 1., 0.),
            Point64::new(-1., 0., 0.),
            Point64::new(1., 0., 0.),
            Material::Dielectric(Dielectric {
                index_of_refraction: 1.0,
            }),
        );

        let r = Ray {
            origin: Point64::new(1., -1., -2.),
            direction: Point64::new(0., 0., 1.),
            exposure_time: 1.0,
        };

        assert!(t.is_hit_by(&r, Interval::new(0.0, 100.0)).is_none());
    }

    #[test]
    fn ray_hits_triangle() {
        let t = Triangle::new(
            Point64::new(0., 1., 0.),
            Point64::new(-1., 0., 0.),
            Point64::new(1., 0., 0.),
            Material::Dielectric(Dielectric {
                index_of_refraction: 1.0,
            }),
        );

        let r = Ray {
            origin: Point64::new(0., 0.5, -2.),
            direction: Point64::new(0., 0., 1.),
            exposure_time: 1.0,
        };

        let hr = t.is_hit_by(&r, Interval::new(0.0, 100.0)).unwrap();
        assert_eq!(hr.value, 2.0)
    }

    fn unit_triangle() -> Triangle {
        Triangle::new(
            Point64::new(0., 1., 0.),
            Point64::new(-1., 0., 0.),
            Point64::new(1., 0., 0.),
            Material::Dielectric(Dielectric {
                index_of_refraction: 1.0,
            }),
        )
    }

    #[test]
    fn triangle_behind_ray_is_not_hit() {
        // Triangle sits at z = 0; ray starts at z = 2 and points further away (+z).
        let r = Ray {
            origin: Point64::new(0., 0.5, 2.),
            direction: Point64::new(0., 0., 1.),
            exposure_time: 1.0,
        };
        assert!(
            unit_triangle()
                .is_hit_by(&r, Interval::new(0.001, f64::INFINITY))
                .is_none()
        );
    }

    #[test]
    fn triangle_beyond_t_max_is_not_hit() {
        // Hit would be at t = 2, but a closer object already capped the search at t = 1.
        let r = Ray {
            origin: Point64::new(0., 0.5, -2.),
            direction: Point64::new(0., 0., 1.),
            exposure_time: 1.0,
        };
        assert!(
            unit_triangle()
                .is_hit_by(&r, Interval::new(0.001, 1.0))
                .is_none()
        );
    }
}
