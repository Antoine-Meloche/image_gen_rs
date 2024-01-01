use image::{ImageBuffer, RgbImage};

pub fn create_image(height: u32, width: u32) -> RgbImage {
    let image: RgbImage = ImageBuffer::new(width, height);
    return image;
}

pub fn edit_pixel(image: &mut RgbImage, x: u32, y: u32, red: u8, green: u8, blue: u8) {
    if x >= image.width() || y >= image.height() {
        return
    }

    *image.get_pixel_mut(x, y) = image::Rgb([red, green, blue]);
}

pub fn save_image(image: RgbImage) {
    image.save("output.png").unwrap();
}

pub fn draw_circle(image: &mut RgbImage, x: u32, y: u32, size: u32, red: u8, green: u8, blue: u8) {
    let radius   = size / 2;
    let center_x = x + radius;
    let center_y = y + radius;
    for i in 0..360 {
        let dx = center_x as i32 + (f32::cos((i as f32).to_radians()) * radius as f32) as i32;
        let dy = center_y as i32 + (f32::sin((i as f32).to_radians()) * radius as f32) as i32;

        edit_pixel(image, dx.try_into().unwrap(), dy.try_into().unwrap(), red, green, blue);
    }
}
