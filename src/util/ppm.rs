use image::RgbImage;
use std::io;
use std::io::Write;

#[allow(dead_code)]
pub fn image_to_ppm(image: &RgbImage, file: &mut dyn Write) -> anyhow::Result<()> {
    file.write_all("P3\n".as_bytes())?;
    file.write_all(format!("{} {}\n255\n", image.width(), image.height()).as_bytes())?;

    for pixel in image.pixels() {
        let [r, g, b] = pixel.0;
        file.write_all(format!("{} {} {}\n", r, g, b).as_bytes())?;
    }

    Ok(())
}

#[allow(dead_code)]
pub fn main() -> anyhow::Result<()> {
    let image_width = 256;
    let image_height = 256;

    let mut img = RgbImage::new(image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let r = i as f64 / ((image_width - 1) as f64);
            let g = j as f64 / ((image_height - 1) as f64);
            let b = 0.0;

            img.put_pixel(
                i,
                j,
                image::Rgb::from([
                    (r * 255.0).floor() as u8,
                    (g * 255.0f64).floor() as u8,
                    (b * 255.0f64).floor() as u8,
                ]),
            );
        }
    }

    image_to_ppm(&img, &mut io::stdout())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn writes_p3_header_and_pixels_in_row_order() {
        let mut img = RgbImage::new(2, 2);
        img.put_pixel(0, 0, image::Rgb([255, 0, 0]));
        img.put_pixel(1, 0, image::Rgb([0, 255, 0]));
        img.put_pixel(0, 1, image::Rgb([0, 0, 255]));
        img.put_pixel(1, 1, image::Rgb([1, 2, 3]));

        let mut out = Vec::new();
        image_to_ppm(&img, &mut out).unwrap();

        assert_eq!(
            String::from_utf8(out).unwrap(),
            "P3\n2 2\n255\n255 0 0\n0 255 0\n0 0 255\n1 2 3\n"
        );
    }
}
