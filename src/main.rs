use crate::color64::Color64;
use crate::point64::Point64;
use crate::ray::Ray;
use crate::sphere::Sphere;

mod color64;
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

    let sphere = Sphere {
        center: Point64::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    print!("P3\n{} {}\n255\n", width, height);

    let color_ref_point = Point64::new(0.0, 0.0, -1.0);

    for j in (0..height).rev() {
        for i in 0..width {
            let u = i as f64 / width as f64;
            let v = j as f64 / height as f64;

            let direction = Point64(*lower_left_corner + u * *horizontal + v * *vertical);

            let ray = Ray {
                origin: &origin,
                direction: &direction,
            };

            let hit_magnitude = sphere.hit_magnitude(&ray);

            let color = if hit_magnitude > 0.0 {
                let point_to_ref =
                    Point64(*ray.point_at_parameter(hit_magnitude) - *color_ref_point);

                Color64::new(
                    0.5 * (point_to_ref.x() + 1.0),
                    0.5 * (point_to_ref.y() + 1.0),
                    0.5 * (point_to_ref.z() + 1.0),
                )
            } else {
                color_of_space(&ray)
            };

            let scaled_red = (255.99 * color.r()) as i32;
            let scaled_green = (255.99 * color.g()) as i32;
            let scaled_blue = (255.99 * color.b()) as i32;

            println!("{} {} {}", scaled_red, scaled_green, scaled_blue);
        }
    }
}
