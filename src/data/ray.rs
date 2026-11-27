use crate::data::background::Background;
use crate::data::color64::{BLACK, Color64};
use crate::data::point64::Point64;
use crate::hittables::Hittable;
use rand::Rng;
use std::ops::Add;

pub struct Ray {
    pub origin: Point64,
    pub direction: Point64,
    pub exposure_time: f64,
}

/// After this many bounces, use Russian roulette to terminate diffuse paths.
const RR_MIN_BOUNCES: i32 = 3;

impl Add for Color64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color64(self.0 + rhs.0)
    }
}

impl Ray {
    pub fn point_at_parameter(&self, t: f64) -> Point64 {
        self.origin + self.direction * t
    }

    pub fn color_in_world(
        &self,
        world: &Hittable,
        background: &Background,
        max_depth: i32,
        rng: &mut impl Rng,
    ) -> Color64 {
        self.color_in_world_recurse(world, background, max_depth, max_depth, rng)
    }

    fn color_in_world_recurse(
        &self,
        world: &Hittable,
        background: &Background,
        max_depth: i32,
        depth: i32,
        rng: &mut impl Rng,
    ) -> Color64 {
        if depth < 1 {
            return BLACK;
        }

        let hit_record = world.is_hit_by(self, 0.001, f64::INFINITY);

        match hit_record {
            Some(hit_record) => {
                let emitted =
                    hit_record
                        .material
                        .emitted(hit_record.u, hit_record.v, &hit_record.location);
                match hit_record.material.scatter(self, &hit_record) {
                    Some(scatter_record) => {
                        let bounce = max_depth - depth;
                        let mut att = scatter_record.attenuation;
                        if bounce >= RR_MIN_BOUNCES {
                            let p = att.r().max(att.g()).max(att.b()).clamp(0.001, 1.0);
                            if rng.random::<f64>() > p {
                                return emitted;
                            }
                            att = Color64(att.0 / p);
                        }
                        emitted
                            + att.component_mul(&scatter_record.scattered.color_in_world_recurse(
                                world,
                                background,
                                max_depth,
                                depth - 1,
                                rng,
                            ))
                    }

                    None => emitted,
                }
            }

            None => background.color(self),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn point_at_parameter() {
        let r = Ray {
            origin: Point64::new(1., 2., 3.),
            direction: Point64::new(0., 1., -2.),
            exposure_time: 0.,
        };
        assert_abs_diff_eq!(r.point_at_parameter(0.).0, Point64::new(1., 2., 3.).0);
        assert_abs_diff_eq!(r.point_at_parameter(2.5).0, Point64::new(1., 4.5, -2.).0);
        assert_abs_diff_eq!(r.point_at_parameter(-1.).0, Point64::new(1., 1., 5.).0);
    }
}
