mod util;
mod mandelbrot;
use image::{Rgb, RgbImage};
use show_image::create_window;

use crate::{mandelbrot::ColorMap, util::to_showable_image};
use text_io::read;

enum Color {
    WHITE,
    BLACK 
}

impl Color {
    // Method to get the RGBA color value for the chess board
    fn pixel(&self) -> Rgb<u8> {
        match *self {
            Color::WHITE => Rgb([255, 255, 255]),
            Color::BLACK => Rgb([0, 0, 0]),
        }
    }
}

fn get_board_color(x: u32, y: u32, board_width: u32) -> Rgb<u8> {
    let actual_square_size = board_width as f32 / 8.0;

    // Determine the row and column of the square based on pixel coordinates
    let row = (y as f32 / actual_square_size) as u32;
    let col = (x as f32 / actual_square_size) as u32;

    // If the sum of row and col is even, it's a white square, otherwise black
    if (row + col) % 2 == 0 {
        Color::WHITE.pixel()
    } else {
        Color::BLACK.pixel()
    }
}

fn chessboard_exercice() -> RgbImage {
    // Prompt the user to enter the size of the board
    println!("Enter the board size:");
    let cell_count: u32 = read!();

    // Create a new RgbImage with the given board size
    let mut image = RgbImage::new(cell_count, cell_count);
    for x in 0..cell_count {
        for y in 0..cell_count {
            image.put_pixel(x, y, get_board_color(x,y, cell_count));
        }
    }
    return image;
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_board_color_8by8() {
        // Test for a 8x8 chessboard
        assert_eq!(get_board_color(0, 0, 8), Color::WHITE.pixel());
        assert_eq!(get_board_color(1, 0, 8), Color::BLACK.pixel());
        assert_eq!(get_board_color(0, 1, 8), Color::BLACK.pixel());
        assert_eq!(get_board_color(1, 1, 8), Color::WHITE.pixel());
        assert_eq!(get_board_color(4, 4, 8), Color::WHITE.pixel());
        assert_eq!(get_board_color(7, 7, 8), Color::WHITE.pixel());
    }
    #[test]
    fn test_get_board_color_40by40() {
        // Test for a 40x40 chessboard with some coordinates
        assert_eq!(get_board_color(0, 0, 40), Color::WHITE.pixel());
        assert_eq!(get_board_color(5, 5, 40), Color::WHITE.pixel());
        assert_eq!(get_board_color(39, 0, 40), Color::BLACK.pixel());
        assert_eq!(get_board_color(0, 39, 40), Color::BLACK.pixel());
        assert_eq!(get_board_color(16, 21, 40), Color::BLACK.pixel());
        assert_eq!(get_board_color(31, 31, 40), Color::WHITE.pixel());
        assert_eq!(get_board_color(39, 39, 40), Color::WHITE.pixel());
    }
}


fn mandelbrot_plot(
                    width: u32, 
                    height: u32, 
                    mandelbrot_x_min: f64, 
                    mandelbrot_x_max: f64, 
                    mandelbrot_y_min: f64,
                    mandelbrot_y_max: f64,
                    cmap: Box<dyn ColorMap> ) -> RgbImage {
    
    let mut image = RgbImage::new(width, height);
    let max_iteration = 100;  

    for px in 0..width {
        for py in 0..height {
            // Scale pixel coordinates to Mandelbrot coordinates
            let x0 = mandelbrot_x_min + (px as f64 / width as f64) * (mandelbrot_x_max - mandelbrot_x_min);
            let y0 = mandelbrot_y_min + (py as f64 / height as f64) * (mandelbrot_y_max - mandelbrot_y_min);
            
            let mut x = 0.0;
            let mut y = 0.0;
            let mut iteration = 0;

            while x * x + y * y <= 2.0 * 2.0 && iteration < max_iteration {
                let xtemp = x * x - y * y + x0;
                y = 2.0 * x * y + y0;
                x = xtemp;
                iteration += 1;
            }

            image.put_pixel(px, py, cmap.color(iteration));
        }
    }
    return image;

}


fn mandelbrot_exercice() -> anyhow::Result<RgbImage> {
    let width = 1024;
    let height = 1024;

    // Prompt the user to enter the space coordinates
    println!("enter the space coordinates");
    let input_string: String = read!("{}\n");
    
    // Split the input string into individual parts
    let parts: Vec<&str> = input_string.split(';').collect();
    
    // Extract individual parts into variables, setting defaults if parsing fails
    let mandelbrot_x_min: f64 = match parts.get(0) {
        Some(s) => s.parse().unwrap_or(-2.0),
        None => -2.0,
    };
    let mandelbrot_x_max: f64 = match parts.get(1) {
        Some(s) => s.parse().unwrap_or(2.0),
        None => 2.0,
    };
    let mandelbrot_y_min: f64 = match parts.get(2) {
        Some(s) => s.parse().unwrap_or(-1.5),
        None => -1.5,
    };
    let mandelbrot_y_max: f64 = match parts.get(3) {
        Some(s) => s.parse().unwrap_or(1.5),
        None => 1.5,
    };

    println!("Drawing on set x [{},{}] and y [{},{}]", mandelbrot_x_min, mandelbrot_x_max, mandelbrot_y_min, mandelbrot_y_max);
    
    // Prompt the user to enter the colormap
    println!("enter the colormap ['c' or 'gs'] default: gs");
    let input_string: String = read!("{}\n");

    // Select the appropriate colormap based on user input
    let maper: Box<dyn ColorMap> = match input_string.as_str() {
        "c" => Box::new(mandelbrot::ColoredColorMap {
            max_iterations: 100, // Maximum iterations for the Mandelbrot calculation
            colorgrad: colorgrad::turbo(), // Color gradient for colored colormap
        }),
        _ => Box::new(mandelbrot::GrayMap {
            max_iterations: 100, // Maximum iterations for the Mandelbrot calculation
        }),
    };

    // Generate the Mandelbrot image
    let image = mandelbrot_plot(
        width, height, 
        mandelbrot_x_min, mandelbrot_x_max, mandelbrot_y_min, mandelbrot_y_max,
        maper
    );
    
    return Ok(image);
}

#[show_image::main]
fn main() -> anyhow::Result<()> {
    let image = mandelbrot_exercice();
    //let image  = image::RgbImage::new(500, 500);
    // Create a window with default options and display the image.
    let window = create_window("image", Default::default())?;
    window.set_image("image-001", to_showable_image(&image.unwrap()))?;
    window.wait_until_destroyed()?;
    Ok(())
}

