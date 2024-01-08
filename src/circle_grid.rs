use image::RgbImage;

use crate::image_ops::draw_circle;

pub fn create_circle_grid(image: &mut RgbImage) {
    const GRID_SIZE: u32 = 100;

    let red   = 255;
    let green = 255;
    let blue  = 255;

    for mut x in 0..(image.width()/GRID_SIZE) {
        x = x * GRID_SIZE;
        for mut y in 0..(image.height()/GRID_SIZE) {
            y = y * GRID_SIZE;
            // edit_pixel(image, x, y, 255, 255, 255);
            draw_circle(image, x, y, GRID_SIZE, red, green, blue);
        }
    }
}
