mod image_processing;
mod image_viewer;
mod utility;

use image_processing::filters::{ImageFilter, PrewittFilter, SobelFilter};
use image_processing::image_processor::ImageProcessor;
use image_viewer::image_viewer::ImageViewer;
use std::thread;
use utility::Utility;

fn main() {
    let files = Utility::list_input_output_image_files();

    let mut handles = vec![];
    for (input, _) in files {
        let handle = thread::spawn(move || {
            let (input_pixels, dimensions) = Utility::image_file_to_rgb(&input);
            let grayscale_pixels = Utility::convert_rgb_to_grayscale(&input_pixels);
            let image_processor = ImageProcessor::new(&grayscale_pixels, dimensions);
            let sobel_filter_output = image_processor.process(SobelFilter::get_kernel());
            let prewitt_filter_output = image_processor.process(PrewittFilter::get_kernel());
            let output_pixels = vec![
                Utility::convert_grayscale_to_rgb(&sobel_filter_output),
                Utility::convert_grayscale_to_rgb(&prewitt_filter_output),
            ];
            let mut window = ImageViewer::new(input_pixels, output_pixels, dimensions);
            window.run();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}
