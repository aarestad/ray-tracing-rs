pub(crate) mod solid_color;
pub(crate) mod checker;

use crate::data::point64::Point64;
use crate::data::color64::Color64;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, point: &Point64) -> Color64;
}