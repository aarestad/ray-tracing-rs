use std::io;
use std::io::Write;
use image::RgbImage;

pub fn image_to_ppm(image: &RgbImage, file: &mut dyn Write) -> io::Result<()> {
    file.write_all("P3\n".as_bytes())?;
    file.write_all(format!("{} {}\n255\n", image.width(), image.height()).as_bytes())?;

    for pixel in image.pixels() {
        let [r, g, b] = pixel.0;
        file.write_all(format!("{} {} {}\n", r, g, b).as_bytes())?;
    }

    Ok(())
}

pub fn main() -> io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    let mut img = RgbImage::new(image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let r = i as f64 / ((image_width - 1) as f64);
            let g = j as f64 / ((image_height - 1) as f64);
            let b = 0.0;

            img.put_pixel(i, j, image::Rgb::from([(r * 255.0).floor() as u8,
                (g * 255.0f64).floor() as u8,
                (b * 255.0f64).floor() as u8]));
        }
    }

    image_to_ppm(&img, &mut io::stdout())
}