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

    pub fn color_in_world(&self, world: &dyn Hittable) -> Color64 {
        self.color_in_world_recurse(world, MAX_DEPTH)
    }

    fn color_in_world_recurse(&self, world: &dyn Hittable, depth: i32) -> Color64 {
        if depth < 1 {
            return BLACK;
        }

        let hit_record = world.is_hit_by(self, 0.01, f64::INFINITY);

        match hit_record {
            Some(hit_record) => match hit_record.material.scatter(self, &hit_record) {
                Some(scatter_record) => Color64(
                    *(scatter_record.attenuation)
                        * *scatter_record
                            .scattered
                            .color_in_world_recurse(world, depth - 1),
                ),
                None => BLACK,
            },

            None => {
                let unit_direction = Point64((*self.direction).normalized());
                let color_factor = 0.5 * (unit_direction.y() + 1.);
                let white_amt = (1. - color_factor) * *WHITE;
                let blue_amt = color_factor * *LIGHT_BLUE;
                Color64(white_amt + blue_amt)
            }
        }
    }
}
