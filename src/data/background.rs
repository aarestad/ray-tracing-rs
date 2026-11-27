use crate::data::color64::Color64;
use crate::data::ray::Ray;

/// What a ray sees when it escapes the scene without hitting anything.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Background {
    /// A constant color (book 2 onward).
    Solid(Color64),
    /// The book-1 sky: lerp from white (looking down) to light blue (looking up)
    /// based on the ray direction's y component.
    SkyGradient,
}

const SKY_WHITE: Color64 = Color64::new(1.0, 1.0, 1.0);
const SKY_BLUE: Color64 = Color64::new(0.5, 0.7, 1.0);

impl Background {
    pub fn color(&self, ray: &Ray) -> Color64 {
        match self {
            Background::Solid(c) => *c,
            Background::SkyGradient => {
                let unit_y = ray.direction.0.normalize().y;
                let a = 0.5 * (unit_y + 1.0);
                Color64((1.0 - a) * SKY_WHITE.0 + a * SKY_BLUE.0)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::point64::Point64;
    use approx::assert_abs_diff_eq;

    fn ray_toward(x: f64, y: f64, z: f64) -> Ray {
        Ray {
            origin: Point64::new(0., 0., 0.),
            direction: Point64::new(x, y, z),
            exposure_time: 0.,
        }
    }

    #[test]
    fn solid_ignores_direction() {
        let c = Color64::new(0.1, 0.2, 0.3);
        assert_eq!(Background::Solid(c).color(&ray_toward(0., 1., 0.)), c);
        assert_eq!(Background::Solid(c).color(&ray_toward(0., -1., 0.)), c);
    }

    #[test]
    fn sky_gradient_endpoints() {
        let up = Background::SkyGradient.color(&ray_toward(0., 5., 0.));
        let down = Background::SkyGradient.color(&ray_toward(0., -5., 0.));
        let level = Background::SkyGradient.color(&ray_toward(0., 0., -1.));
        assert_abs_diff_eq!(up.0, SKY_BLUE.0, epsilon = 1e-12);
        assert_abs_diff_eq!(down.0, SKY_WHITE.0, epsilon = 1e-12);
        assert_abs_diff_eq!(level.0, (0.5 * (SKY_WHITE.0 + SKY_BLUE.0)), epsilon = 1e-12);
    }
}
