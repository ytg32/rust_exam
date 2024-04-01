use image::{flat::SampleLayout, RgbImage};
use show_image::{glam::UVec2, ImageInfo};

/**
 * This method allow to convert a modifiable image (from the image crate) to a showable image (from the show_image crate).
 */
pub fn to_showable_image(image: &RgbImage) -> show_image::ImageView {
    let samples = image.as_flat_samples();
    show_image::ImageView::new(get_image_info(&samples.layout), samples.samples)
}

fn get_image_info(layout: &SampleLayout) -> ImageInfo {
    ImageInfo {
        size: UVec2::new(layout.width, layout.height),
        stride: UVec2::new(layout.width_stride as u32, layout.height_stride as u32),
        pixel_format: show_image::PixelFormat::Rgb8,
    }
}
