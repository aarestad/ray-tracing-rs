use crate::data::color64::{Color64, BLACK, LIGHT_BLUE, WHITE};
use crate::data::point64::Point64;
use crate::hittables::Hittable;

pub struct Ray {
    pub origin: Point64,
    pub direction: Point64,
    pub exposure_time: f64,
}

const MAX_DEPTH: i32 = 50;

impl Ray {
    pub fn point_at_parameter(&self, t: f64) -> Point64 {
        Point64(*self.origin + t * *self.direction)
    }

    pub fn color_in_world(&self, world: &dyn Hittable, background: &Color64) -> Color64 {
        self.color_in_world_recurse(world, background, MAX_DEPTH)
    }

    fn color_in_world_recurse(
        &self,
        world: &dyn Hittable,
        background: &Color64,
        depth: i32,
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
                    Some(scatter_record) => Color64(
                        *emitted
                            + (*scatter_record.attenuation
                                * *scatter_record.scattered.color_in_world_recurse(
                                    world,
                                    background,
                                    depth - 1,
                                )),
                    ),
                    None => emitted,
                }
            }

            // None => {
            //     let unit_direction = Point64((*self.direction).normalized());
            //     let color_factor = 0.5 * (unit_direction.y() + 1.);
            //     let white_amt = (1. - color_factor) * *WHITE;
            //     let blue_amt = color_factor * *LIGHT_BLUE;
            //     Color64(white_amt + blue_amt)
            // }
            None => *background,
        }
    }
}
