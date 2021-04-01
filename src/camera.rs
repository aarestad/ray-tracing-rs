use crate::point64::Point64;

pub struct Camera {
    pub origin: Point64,
    pub lower_left_corner: Point64,
    pub horizontal: Point64,
    pub vertical: Point64,
}

impl Camera {
    pub fn new(
        vfov_deg: f64, // vertical field ovf view
        aspect_ratio: f64,
    ) -> Camera {
        let h = (vfov_deg.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let focal_length = 1.0;

        let origin = Point64::new(0.0, 0.0, 0.0);
        let horizontal = Point64::new(viewport_width, 0.0, 0.0);
        let vertical = Point64::new(0.0, viewport_height, 0.0);

        let lower_left_corner = Point64(
            *origin - *horizontal / 2.0 - *vertical / 2.0 - *Point64::new(0.0, 0.0, focal_length),
        );

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn direction(&self, u: f64, v: f64) -> Point64 {
        Point64(*self.lower_left_corner + u * *self.horizontal + v * *self.vertical - *self.origin)
    }
}
