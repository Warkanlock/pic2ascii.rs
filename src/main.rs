use image;
use std::io::{stdout, Write};

fn main() {
    // open a default image
    let img = image::open("test.png").unwrap().into_rgb8();

    // store the ascii here
    let mut ascii: String = String::new();

    // symbols we will use in the image generation
    let chars: [&str; 6] = ["@", "%", "*", "+", "-", "."];

    // maximum resolution used by the scalation algorithm
    let mut resolution_max = 8;

    let scaled_image = image::imageops::resize(
        &img,
        img.width() / (resolution_max / 2),
        img.height() / resolution_max,
        image::imageops::FilterType::Nearest,
    );

    for pixel in scaled_image.enumerate_pixels() {
        if resolution_max == pixel.0 {
            ascii.push_str("\n"); // jump to prevent infinite lines
            resolution_max = pixel.0;
        }

        // store data from pixel without coordinates
        let pixel_data = pixel.2;

        // get pixel bright
        let brightness: f32 =
            ((pixel_data[0] as u32 + pixel_data[1] as u32 + pixel_data[2] as u32) / 3) as f32;

        // char we will use at X,Y position
        let char_position = ((brightness / 255.0) * (chars.len() - 1) as f32).round() as usize;

        ascii.push_str(chars[char_position]);
    }

    stdout().write_all(ascii.as_bytes()).unwrap();
}
