use crate::color64::Color64;

mod vec3;
mod ray;
mod point64;
mod color64;


fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;

    print!("P3\n{} {}\n255\n", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let color: Color64 = Color64::new(
                i as f64 / nx as f64,
                j as f64 / ny as f64,
                0.2,
            );

            let scaled_red: i32 = (255.99 * color.r()) as i32;
            let scaled_green: i32 = (255.99 * color.g()) as i32;
            let scaled_blue: i32 = (255.99 * color.b()) as i32;

            print!("{} {} {}\n", scaled_red, scaled_green, scaled_blue);
        }
    }
}
