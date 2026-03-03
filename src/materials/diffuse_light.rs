use crate::data::color64::Color64;
use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::HitRecord;
use crate::materials::{Material, ScatterRecord};
use crate::textures::Textures;

pub struct DiffuseLight {
    emitter: Textures,
}

impl DiffuseLight {
    pub fn new(color: Color64) -> Self {
        Self {
            emitter: Textures::SolidColor(color),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, u: f64, v: f64, point: &Point64) -> Color64 {
        self.emitter.value(u, v, point)
    }
}
