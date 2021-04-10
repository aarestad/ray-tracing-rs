use crate::data::color64::Color64;
use crate::data::point64::Point64;
use crate::textures::Texture;
use std::sync::Arc;

struct Checker {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, point: &Point64) -> Color64 {
        let sines = (10. * point.x()).sin() * (10. * point.y()).sin() * (10. * point.z()).sin();

        if sines < 0. {
            self.odd.value(u, v, point)
        } else {
            self.even.value(u, v, point)
        }
    }
}
