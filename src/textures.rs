use crate::data::color64::Color64;
use crate::data::point64::Point64;

pub(crate) mod checker;
pub(crate) mod image;
pub(crate) mod noise;
pub(crate) mod perlin;
pub(crate) mod solid_color;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, point: &Point64) -> Color64;
}
