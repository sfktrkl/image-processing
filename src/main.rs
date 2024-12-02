mod image_processing;
mod image_viewer;
mod utility;

use image_processing::filters::{
    CannyFilter, GaussianBlur, ImageFilter, PrewittFilter, SobelFilter,
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
    let output = process_image(&input, dimensions, &kernels);
    (input, output, dimensions)
}

fn process_image(
    input: &[u32],
    dimensions: (u32, u32),
    filters: &[Box<dyn ImageFilter>],
) -> Vec<Vec<u32>> {
    let grayscale = Utility::convert_rgb_to_grayscale(input);

    filters
        .iter()
        .map(|filter| {
            let kernel = filter.get_kernel();
            let options = filter.compute_options(&grayscale);
            if kernel.1 == "gaussianBlur" {
                let channels = Utility::decompose_rgb(input);
                let r_processor = ImageProcessor::new(&channels.0, &options, dimensions);
                let r_output = r_processor.process(kernel);
                let g_processor = ImageProcessor::new(&channels.1, &options, dimensions);
                let g_output = g_processor.process(kernel);
                let b_processor = ImageProcessor::new(&channels.2, &options, dimensions);
                let b_output = b_processor.process(kernel);
                Utility::recompose_rgb(&r_output, &g_output, &b_output)
            } else {
                let processor = ImageProcessor::new(&grayscale, &options, dimensions);
                let output = processor.process(kernel);
                Utility::convert_grayscale_to_rgb(&output)
            }
        })
        .collect()
}

fn view_images(input: Vec<u32>, outputs: Vec<Vec<u32>>, dimensions: (u32, u32)) {
    let mut window = Viewer::new(input, outputs, dimensions);
    window.run();
}
