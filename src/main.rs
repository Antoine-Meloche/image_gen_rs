use image::RgbImage;

mod image_ops;
use image_ops::{
    create_image,
    save_image
};

mod circle_grid;
use circle_grid::create_circle_grid;

const WIDTH:  u32 = 1000;
const HEIGHT: u32 = 1000;

fn main() {
    let mut image: RgbImage = create_image(WIDTH, HEIGHT);
    create_circle_grid(&mut image);
    // edit_pixel(&mut image, 5, 5, 255, 255, 255);
    save_image(image);
}
