use crate::color64::Color64;
use crate::hittable::Hittable;
use crate::hittable_vec::HittableVec;
use crate::point64::Point64;
use crate::ray::Ray;
use crate::sphere::Sphere;

mod color64;
mod hittable;
mod hittable_vec;
mod point64;
mod ray;
mod sphere;
mod vec3_64;

const WHITE: Color64 = Color64::new(1.0, 1.0, 1.0);
const LIGHT_BLUE: Color64 = Color64::new(0.5, 0.7, 1.0);

fn color_of_space(ray: &Ray) -> Color64 {
    let unit_direction = Point64((*ray.direction).normalized());
    let color_factor = 0.5 * (unit_direction.y() + 1.0);
    let white_amt = (1.0 - color_factor) * *WHITE;
    let blue_amt = color_factor * *LIGHT_BLUE;
    Color64(white_amt + blue_amt)
}

fn main() {
    let width: i32 = 800;
    let height: i32 = 400;

    let lower_left_corner = Point64::new(-2.0, -1.0, -1.0);
    let horizontal = Point64::new(4.0, 0.0, 0.0);
    let vertical = Point64::new(0.0, 2.0, 0.0);
    let origin = Point64::new(0.0, 0.0, 0.0);

    let s1 = Sphere {
        center: Point64::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    let s2 = Sphere {
        center: Point64::new(0.0, -100.5, -1.0),
        radius: 100.0,
    };

    let hittables = HittableVec {
        hittables: vec![&s1, &s2],
    };

    print!("P3\n{} {}\n255\n", width, height);

    for j in (0..height).rev() {
        for i in 0..width {
            let u = i as f64 / width as f64;
            let v = j as f64 / height as f64;

            let direction = Point64(*lower_left_corner + u * *horizontal + v * *vertical);

            let ray = Ray {
                origin: &origin,
                direction: &direction,
            };

            let hit_record = hittables.is_hit_by(&ray, 0.0, f64::MAX);

            let color = match hit_record {
                Some(hit_record) => Color64::new(
                    0.5 * (hit_record.normal.x() + 1.0),
                    0.5 * (hit_record.normal.y() + 1.0),
                    0.5 * (hit_record.normal.z() + 1.0),
                ),

                None => color_of_space(&ray),
            };

            let scaled_red = (255.99 * color.r()) as i32;
            let scaled_green = (255.99 * color.g()) as i32;
            let scaled_blue = (255.99 * color.b()) as i32;

            println!("{} {} {}", scaled_red, scaled_green, scaled_blue);
        }
    }
}
