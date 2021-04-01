use crate::point64::Point64;
use crate::vec3_64::Vec3_64;
use crate::ray::Ray;

pub struct Camera {
    pub origin: Point64,
    pub lower_left_corner: Point64,
    pub horizontal: Point64,
    pub vertical: Point64,
}

impl Camera {
    pub fn new(
        lookfrom: Point64,
        lookat: Point64,
        vup: Vec3_64,
        vfov_deg: f64, // vertical field ovf view
        aspect_ratio: f64,
    ) -> Camera {
        let h = (vfov_deg.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (*lookfrom - *lookat).normalized();
        let u = vup.cross(&w).normalized();
        let v = w.cross(&u);

        let horizontal = Point64(viewport_width * u);
        let vertical = Point64(viewport_height * v);

        Camera {
            origin: lookfrom,
            horizontal,
            vertical,
            lower_left_corner: Point64(*lookfrom - *horizontal / 2.0 - *vertical / 2.0 - w),
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: Point64(*self.lower_left_corner + s * *self.horizontal + t * *self.vertical - *self.origin),
        }
    }
}
