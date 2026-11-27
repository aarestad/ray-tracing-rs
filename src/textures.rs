use crate::data::color64::Color64;
use crate::data::point64::Point64;

pub(crate) mod image;
pub(crate) mod noise;
pub(crate) mod perlin;
pub(crate) mod solid_color;

use crate::textures::image::ImageTexture;
use crate::textures::noise::Noise;
use crate::textures::solid_color::SolidColor;

#[derive(Clone)]
pub enum Texture {
    Solid(SolidColor),
    /// 3D checker (book 2, section 4.2): cubes of side `scale`, alternating by the parity of
    /// `floor(x / scale) + floor(y / scale) + floor(z / scale)`.
    Checker {
        scale: f64,
        even: Box<Texture>,
        odd: Box<Texture>,
    },
    Noise(Box<Noise>),
    Image(ImageTexture),
}

impl Texture {
    pub fn solid(color: Color64) -> Texture {
        Texture::Solid(SolidColor { color })
    }

    /// Checker of two solid colors, as in the book's `checker_texture(scale, c1, c2)`.
    pub fn checker(scale: f64, even: Color64, odd: Color64) -> Texture {
        Texture::Checker {
            scale,
            even: Box::new(Texture::solid(even)),
            odd: Box::new(Texture::solid(odd)),
        }
    }

    pub fn value(&self, u: f64, v: f64, point: &Point64) -> Color64 {
        match self {
            Texture::Solid(s) => s.color,
            Texture::Checker { scale, even, odd } => {
                let cell = |c: f64| (c / scale).floor() as i64;
                if (cell(point.x()) + cell(point.y()) + cell(point.z())).rem_euclid(2) == 0 {
                    even.value(u, v, point)
                } else {
                    odd.value(u, v, point)
                }
            }
            Texture::Noise(n) => n.value(u, v, point),
            Texture::Image(img) => img.value(u, v, point),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const DARK: Color64 = Color64::new(0.2, 0.3, 0.1);
    const LIGHT: Color64 = Color64::new(0.9, 0.9, 0.9);

    #[test]
    fn checker_alternates_per_cell() {
        let t = Texture::checker(0.5, DARK, LIGHT);
        let at = |x, y, z| t.value(0., 0., &Point64::new(x, y, z));
        assert_eq!(at(0.1, 0.1, 0.1), DARK); // cell (0,0,0): even
        assert_eq!(at(0.6, 0.1, 0.1), LIGHT); // (1,0,0): odd
        assert_eq!(at(0.6, 0.6, 0.1), DARK); // (1,1,0): even
        assert_eq!(at(0.6, 0.6, 0.6), LIGHT); // (1,1,1): odd
    }

    #[test]
    fn checker_handles_negative_coordinates() {
        let t = Texture::checker(1.0, DARK, LIGHT);
        let at = |x, y, z| t.value(0., 0., &Point64::new(x, y, z));
        // floor(-0.5) = -1, so (-1, 0, 0) is odd and (-1, -1, 0) is even.
        assert_eq!(at(-0.5, 0.5, 0.5), LIGHT);
        assert_eq!(at(-0.5, -0.5, 0.5), DARK);
    }
}
