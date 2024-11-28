mod image_processing;
mod utility;

use image_processing::filters::{ImageFilter, SobelFilter};
use image_processing::image_processor::ImageProcessor;
use utility::Utility;

fn main() {
    let files = Utility::list_input_output_image_files();
    for (input, output) in &files {
        let (input_pixels, (width, height)) = Utility::read_image_file(input);
        let sobel_processor = ImageProcessor::new(&input_pixels, width, height);
        let output_pixels = sobel_processor.process(SobelFilter::get_kernel());
        Utility::write_image_file(&output, output_pixels, width, height);
    }
}
