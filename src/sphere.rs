use crate::point64::Point64;
use crate::ray::Ray;

pub struct Sphere {
    pub center: Point64,
    pub radius: f64,
}

impl Sphere {
    pub fn hit_magnitude(&self, ray: &Ray) -> f64 {
        let ray_origin_to_center = **ray.origin - *self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray_origin_to_center.dot(ray.direction);
        let c = ray_origin_to_center.dot(&ray_origin_to_center) - self.radius.powi(2);
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant >= 0.0 {
            (-b - discriminant.sqrt()) / (2.0 * a)
        } else {
            -1.0
        }
    }
}
