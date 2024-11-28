mod image_processing;
mod utility;

use image_processing::filters::{ImageFilter, SobelFilter};
use image_processing::image_processor::ImageProcessor;
use utility::Utility;

fn main() {
    let files = Utility::list_input_output_image_files();
    for (input, output) in &files {
        let (input_pixels, dimensions) = Utility::read_image_file(input);
        let image_processor = ImageProcessor::new(&input_pixels, dimensions);
        let sobel_filter_output = image_processor.process(SobelFilter::get_kernel());
        Utility::write_image_file(&output, sobel_filter_output, dimensions);
    }
}
