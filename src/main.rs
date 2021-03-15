fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;

    print!("P3\n{} {}\n255\n", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let red: f32 = i as f32 / nx as f32;
            let green: f32 = j as f32 / ny as f32;
            let blue: f32 = 0.2;
            let scaled_red: i32 = (255.99 * red) as i32;
            let scaled_green: i32 = (255.99 * green) as i32;
            let scaled_blue: i32 = (255.99 * blue) as i32;
            print!("{} {} {}\n", scaled_red, scaled_green, scaled_blue);
        }
    }
}
