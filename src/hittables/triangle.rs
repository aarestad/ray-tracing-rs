use std::sync::Arc;

use nalgebra::Point;

use super::{axis_aligned_bounding_box::AxisAlignedBoundingBox, HitRecord, Hittable};
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
    pub material: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(p1: Point64, p2: Point64, p3: Point64, material: Arc<dyn Material>) -> Self {
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
            material: material.clone(),
        }
    }
}

impl Hittable for Triangle {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AxisAlignedBoundingBox> {
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

    fn is_hit_by(&self, ray: &Ray, min_value: f64, max_value: f64) -> Option<HitRecord> {
        let cross_e2 = ray.direction.cross(&self.e2);
        let determinant = self.e1.dot(&cross_e2);

        if determinant.abs() < EPSILON {
            return None;
        }

        let f = 1.0 / determinant;
        let p1_to_origin = ray.origin.0 - self.p1.0;
        let u = f * p1_to_origin.dot(&cross_e2);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let origin_cross_e1 = p1_to_origin.cross(&self.e1);
        let v = f * ray.direction.dot(&origin_cross_e1);

        if v < 0.0 || (u + v) > 1.0 {
            return None;
        }

        Some(HitRecord::new(
            f * self.e2.dot(&origin_cross_e1),
            ray,
            Point64(self.normal),
            self.material.clone(),
            (u, v),
        ))
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use approx::assert_abs_diff_eq;

    use crate::{
        data::{point64::Point64, ray::Ray},
        hittables::Hittable,
        materials::dielectric::Dielectric,
    };

    use super::Triangle;

    #[test]
    fn ray_parallel_to_triangle() {
        let t = Triangle::new(
            Point64::new(0., 1., 0.),
            Point64::new(-1., 0., 0.),
            Point64::new(1., 0., 0.),
            Arc::new(Dielectric {
                index_of_refraction: 1.0,
            }),
        );
        let r = Ray {
            origin: Point64::new(0., -1., -2.),
            direction: Point64::new(0., 1., 0.),
            exposure_time: 1.0,
        };
        assert!(t.is_hit_by(&r, 0.0, 100.0).is_none());
    }

    #[test]
    fn ray_misses_p1_p3_edge() {
        let t = Triangle::new(
            Point64::new(0., 1., 0.),
            Point64::new(-1., 0., 0.),
            Point64::new(1., 0., 0.),
            Arc::new(Dielectric {
                index_of_refraction: 1.0,
            }),
        );

        let r = Ray {
            origin: Point64::new(1., -1., -2.),
            direction: Point64::new(0., 0., 1.),
            exposure_time: 1.0,
        };

        assert!(t.is_hit_by(&r, 0.0, 100.0).is_none());
    }

    // #[test]
    // fn ray_misses_p1_p2_edge() {
    //     let t = Triangle::new((
    //         Point64::new(0., 1., 0.),
    //         Point64::new(-1., 0., 0.),
    //         Point64::new(1., 0., 0.),
    //     ));
    //     let r = Ray::new(Point64::new(-1., 1., -2.), Vector::vector(0., 0., 1.));
    //     assert!(t.intersections(&r).ints().is_empty());
    // }

    // #[test]
    // fn ray_misses_p2_p3_edge() {
    //     let t = Triangle::new((
    //         Point64::new(0., 1., 0.),
    //         Point64::new(-1., 0., 0.),
    //         Point64::new(1., 0., 0.),
    //     ));
    //     let r = Ray::new(Point64::new(0., -1., -2.), Vector::vector(0., 0., 1.));
    //     assert!(t.intersections(&r).ints().is_empty());
    // }

    #[test]
    fn ray_hits_triangle() {
        let t = Triangle::new(
            Point64::new(0., 1., 0.),
            Point64::new(-1., 0., 0.),
            Point64::new(1., 0., 0.),
            Arc::new(Dielectric {
                index_of_refraction: 1.0,
            }),
        );

        let r = Ray {
            origin: Point64::new(0., 0.5, -2.),
            direction: Point64::new(0., 0., 1.),
            exposure_time: 1.0,
        };

        let hr = t.is_hit_by(&r, 0.0, 100.0).unwrap();
        assert_eq!(hr.value, 2.0)
    }
}
