use crate::data::color64::Color64;
use crate::textures::Texture;
use crate::data::point64::Point64;
use std::sync::Arc;

pub struct SolidColor {
    pub color: Color64,
}

impl SolidColor {
    pub(crate) fn arc_from(color: Color64) -> Arc<Self> {
        Arc::from(Self {
            color,
        })
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _point: &Point64) -> Color64 {
        self.color
    }
}
