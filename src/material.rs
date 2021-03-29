use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color64::Color64;

struct ScatterRecord {
    hit_record: HitRecord,
    color: Color64,
    scattered: Ray,
}

trait Material {
    fn scatter(ray_in: &Ray) -> Option<ScatterRecord>;
}