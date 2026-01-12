use image::{
    ColorType, DynamicImage, GenericImage, GenericImageView, ImageBuffer, ImageResult, Pixel, Rgba,
};

pub fn color_image(image_content: &mut DynamicImage, color: Rgba<u8>, opacity: f32) {
    let (w, h) = image_content.dimensions();

    for i in 0..w {
        for j in 0..h {
            let mut pixel = image_content.get_pixel(i, j);

            let channels = pixel.channels();

            pixel.0[0] = (color.0[0] as f32 * opacity) as u8;
            pixel.0[1] = (color.0[1] as f32 * opacity) as u8;
            pixel.0[2] = (color.0[2] as f32 * opacity) as u8;
            pixel.0[3] = color.0[3];

            image_content.put_pixel(i, j, pixel);
        }
    }
}

// println!(
//     "color: {} {} {} {}",
//     color.0[0], color.0[1], color.0[2], color.0[3]
// );

pub fn print_image(image_content: DynamicImage) {
    let content = image_content.pixels();

    for (x, y, pixel) in content {
        let channels = pixel.0;

        println!("Pixel at ({}, {}): {:?}", x, y, channels);
    }
}

pub fn save_image(image_content: DynamicImage, path: &str) {
    match image_content.save(path) {
        Ok(_) => {}
        Err(e) => println!("Error saving: {}", e),
    }
}
