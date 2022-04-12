use crate::data::color64::Color64;
use crate::data::point64::Point64;
use crate::textures::Texture;
use std::sync::Arc;

pub struct Checker {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

// Larger scale -> smaller squares
const CHECKER_SCALE: f64 = 10.;

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, point: &Point64) -> Color64 {
        let sines = (CHECKER_SCALE * point.x()).sin()
            * (CHECKER_SCALE * point.y()).sin()
            * (CHECKER_SCALE * point.z()).sin();

        if sines < 0. {
            self.odd.value(u, v, point)
        } else {
            self.even.value(u, v, point)
        }
    }
}
