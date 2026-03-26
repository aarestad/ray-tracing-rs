use crate::data::color64::Color64;
use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::HitRecord;
use crate::materials::ScatterRecord;
use crate::textures::Texture;
use std::sync::Arc;

pub struct DiffuseLight {
    emitter: Arc<Texture>,
}

impl DiffuseLight {
    pub fn new(color: Color64) -> Self {
        Self {
            emitter: Texture::arc_solid(color),
        }
    }

    pub fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    pub fn emitted(&self, u: f64, v: f64, point: &Point64) -> Color64 {
        self.emitter.value(u, v, point)
    }
}
