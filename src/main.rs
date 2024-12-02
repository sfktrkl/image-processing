mod image_processing;
mod image_viewer;
mod utility;

use image_processing::filters::{CannyFilter, ImageFilter, PrewittFilter, SobelFilter};
use image_processing::image_processor::ImageProcessor;
use image_viewer::Viewer;
use utility::Utility;

fn main() {
    let files = Utility::list_input_output_image_files();
    let kernels = vec![
        SobelFilter::get_kernel(),
        PrewittFilter::get_kernel(),
        CannyFilter::get_kernel(),
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

fn prepare_images(file: &str, kernels: &[(&str, &str)]) -> (Vec<u32>, Vec<Vec<u32>>, (u32, u32)) {
    let (input, dimensions) = Utility::image_file_to_rgb(&file);
    let output = process_image(&input, dimensions, &kernels);
    (input, output, dimensions)
}

fn process_image(input: &[u32], dimensions: (u32, u32), kernels: &[(&str, &str)]) -> Vec<Vec<u32>> {
    let grayscale = Utility::convert_rgb_to_grayscale(input);
    let options = vec![];

    kernels
        .iter()
        .map(|&kernel| {
            let processor = ImageProcessor::new(&grayscale, &options, dimensions);
            let output = processor.process(kernel);
            Utility::convert_grayscale_to_rgb(&output)
        })
        .collect()
}

fn view_images(input: Vec<u32>, outputs: Vec<Vec<u32>>, dimensions: (u32, u32)) {
    let mut window = Viewer::new(input, outputs, dimensions);
    window.run();
}
