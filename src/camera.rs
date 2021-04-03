#![allow(clippy::too_many_arguments)]

use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::data::vec3_64::Vec3_64;
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Point64,
    pub lower_left_corner: Point64,
    pub horizontal: Point64,
    pub vertical: Point64,
    pub u: Point64,
    pub v: Point64,
    pub w: Point64,
    pub lens_radius: f64,
    pub exposure_time0: f64,
    pub exposure_time1: f64,
}

impl Camera {
    pub fn new(
        look_from: Point64,
        look_at: Point64,
        vup: Vec3_64,
        vfov_deg: f64, // vertical field ovf view
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        exposure_time0: f64,
        exposure_time1: f64,
    ) -> Camera {
        let h = (vfov_deg.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Point64((*look_from - *look_at).normalized());
        let u = Point64(vup.cross(&w).normalized());
        let v = Point64(w.cross(&u));

        let horizontal = Point64(viewport_width * *u * focus_dist);
        let vertical = Point64(viewport_height * *v * focus_dist);

        Camera {
            origin: look_from,
            horizontal,
            vertical,
            u,
            v,
            w,
            lower_left_corner: Point64(
                *look_from - *horizontal / 2.0 - *vertical / 2.0 - *w * focus_dist,
            ),
            lens_radius: aperture / 2.0,
            exposure_time0,
            exposure_time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Point64(self.lens_radius * Vec3_64::random_in_unit_disk());
        let offset = *self.u * rd.x() + *self.v * rd.y();

        Ray {
            origin: Point64(*self.origin + offset),
            direction: Point64(
                *self.lower_left_corner + s * *self.horizontal + t * *self.vertical
                    - *self.origin
                    - offset,
            ),
            exposure_time: rand::thread_rng().gen_range(self.exposure_time0..self.exposure_time1),
        }
    }
}
