mod util;
use image::RgbImage;
use show_image::create_window;

use crate::util::to_showable_image;


fn chessboard_exercice() -> RgbImage {
    todo!()
}

fn mandelbrot_exercice() -> anyhow::Result<RgbImage> {
    todo!()
}

#[show_image::main]
fn main() -> anyhow::Result<()> {
    let image  = image::RgbImage::new(500, 500);
    // Create a window with default options and display the image.
    let window = create_window("image", Default::default())?;
    window.set_image("image-001", to_showable_image(&image))?;
    window.wait_until_destroyed()?;
    Ok(())
}
