use image::{GenericImageView, ImageReader, Rgba};

use std::env;

use oxim::{color_image, print_image};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Not enough arguments");
        return Err("Not enough arguments".into());
    }

    let image_path = &args[1];

    println!("Image: {}", image_path);

    let image = match ImageReader::open(image_path) {
        Ok(img) => img,
        Err(e) => {
            println!("Failed to decode image: {}", e);
            return Err(Box::new(e));
        }
    };

    let mut image_content = match image.decode() {
        Ok(img) => img,
        Err(e) => {
            println!("Failed to decode image: {}", e);
            return Err(Box::new(e));
        }
    };

    let (w, h) = image_content.dimensions();

    println!("image dimensions: {:?} {:?}", w, h);

    let pixels = image_content.pixels();

    // for (x, y, pixel) in image_content.pixels() {
    //     let channels = pixel.0;

    //     println!("Pixel at ({}, {}): {:?}", x, y, channels);
    // }
    //

    let color: Rgba<u8> = Rgba([255, 0, 0, 255]);
    oxim::color_image(&mut image_content, color, 0.5);

    oxim::print_image(image_content.clone());

    oxim::save_image(image_content, "output.png");

    Ok(())
}
