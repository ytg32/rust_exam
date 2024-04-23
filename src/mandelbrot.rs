use colorgrad::Gradient;
use image::Rgb;

// ColorMap trait allow to define a color map for the mandelbrot set
// The color method should return the color of the pixel for an iteration "i" between 0 and max_iterations.
pub trait ColorMap {
    fn color(&self, i: u32) -> image::Rgb<u8>;
    fn get_max_iterations(&self) -> u32;
}


pub struct ColoredColorMap {
    pub(crate) max_iterations: u32,
    pub(crate) colorgrad: Gradient
} 

impl ColorMap for ColoredColorMap {
    fn color(&self, i: u32) -> image::Rgb<u8> {
        if i == self.max_iterations {
            return Rgb([0, 0, 0]); // Black for points inside the set
        }
        //  Calculate the color using the gradient based on the iteration count
        let rgba_color = self.colorgrad.at(i as f64 / self.max_iterations as f64).to_rgba8();
        image::Rgb([rgba_color[0], rgba_color[1], rgba_color[2]])
    }

    fn get_max_iterations(&self) -> u32 {
        self.max_iterations
    }
}

pub struct GrayMap {
    pub(crate) max_iterations: u32,
}

impl ColorMap for GrayMap {
    fn color(&self, i: u32) -> image::Rgb<u8> {
        let color = if i == self.max_iterations {
            Rgb([0, 0, 0]) // Black for points inside the set
        } else {
            // Adjust color based on iteration
            let brightness = (i as f64 / self.max_iterations as f64) * 255.0;
            Rgb([brightness as u8, brightness as u8, brightness as u8])
        };
        
        return color;
    }

    fn get_max_iterations(&self) -> u32 {
        self.max_iterations
    }
}