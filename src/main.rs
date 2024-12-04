mod image_processing;
mod image_viewer;
mod utility;

use image_processing::filters::{
    CannyFilter, GaussianBlur, ImageFilter, LaplacianSharpening, PrewittFilter, SobelFilter,
};
use image_processing::image_processor::ImageProcessor;
use image_viewer::Viewer;
use utility::Utility;

fn main() {
    let files = Utility::list_input_output_image_files();
    let kernels: Vec<Box<dyn ImageFilter>> = vec![
        Box::new(SobelFilter),
        Box::new(PrewittFilter),
        Box::new(CannyFilter),
        Box::new(GaussianBlur),
        Box::new(LaplacianSharpening),
    ];

    let mut handles = vec![];
    for (input_file, _) in files {
        let (input, outputs, dimensions) = prepare_images(&input_file, &kernels);

        let handle = std::thread::spawn(move || {
            view_images(input, outputs, dimensions);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

fn prepare_images(
    file: &str,
    kernels: &[Box<dyn ImageFilter>],
) -> (Vec<u32>, Vec<Vec<u32>>, (u32, u32)) {
    let (input, dimensions) = Utility::image_file_to_rgb(&file);
    let processor = ImageProcessor::new(&input, dimensions, &kernels);
    let output = processor.process_image();
    (input, output, dimensions)
}

fn view_images(input: Vec<u32>, outputs: Vec<Vec<u32>>, dimensions: (u32, u32)) {
    let mut window = Viewer::new(input, outputs, dimensions);
    window.run();
}
