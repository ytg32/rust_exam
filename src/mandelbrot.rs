use image::RgbImage;

// ColorMap trait allow to define a color map for the mandelbrot set
// The color method should return the color of the pixel for an iteration "i" between 0 and max_iterations.
pub trait ColorMap {
    fn color(&self, i: u32) -> image::Rgb<u8>;
    fn get_max_iterations(&self) -> u32;
}
