#![allow(clippy::too_many_arguments)]

use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::data::vector3::random_in_unit_disk;
use nalgebra::Vector3;
use rand::Rng;
use std::ops::Range;

pub struct Camera {
    origin: Point64,
    lower_left_corner: Point64,
    horizontal: Point64,
    vertical: Point64,
    uvw: (Point64, Point64, Point64),
    lens_radius: f64,
    exposure_time: Range<f64>,
}

impl Camera {
    pub fn new(
        look_from: Point64,
        look_at: Point64,
        vup: Vector3<f64>,
        vfov_deg: f64, // vertical field of view
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        exposure_time: Range<f64>,
    ) -> Camera {
        let h = (vfov_deg.to_radians() / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Point64((look_from - look_at).0.normalize());
        let u = Point64(vup.cross(&w.0).normalize());
        let v = Point64(w.0.cross(&u.0));

        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;

        Camera {
            origin: look_from,
            horizontal,
            vertical,
            uvw: (u, v, w),
            lower_left_corner: look_from - horizontal / 2. - vertical / 2. - w * focus_dist,
            lens_radius: aperture / 2.,
            exposure_time,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Point64(self.lens_radius * random_in_unit_disk());
        let offset = self.uvw.0 * rd.x() + self.uvw.1 * rd.y();

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset,
            exposure_time: rand::thread_rng().gen_range(self.exposure_time.clone()),
        }
    }
}
