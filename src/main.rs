mod image_processing;

use image::{open, GrayImage, ImageBuffer, Luma};
use image_processing::filters::{ImageFilter, SobelFilter};
use image_processing::image_processor::ImageProcessor;
use std::fs;

fn list_input_output_image_files() -> Vec<(String, String)> {
    let input_folder = "input";
    let output_folder = "output";

    let files = fs::read_dir(input_folder).expect("Failed to read input folder");

    let mut paths = Vec::new();
    for file in files {
        let path = file.expect("Failed to read file").path();
        if let Some(extension) = path.extension() {
            if extension == "jpg" || extension == "png" || extension == "jpeg" {
                if let Some(input_path) = path.to_str() {
                    if let Some(file_name) = path.file_name() {
                        let output_path =
                            format!("{}/{}", output_folder, file_name.to_string_lossy());
                        paths.push((input_path.to_string(), output_path));
                    }
                }
            }
        }
    }

    paths
}

fn read_image_file(file: &str) -> (Vec<f32>, (u32, u32)) {
    let img = open(file).expect("Failed to open image").to_luma8();
    (
        img.pixels().map(|p| p[0] as f32).collect(),
        img.dimensions(),
    )
}

fn write_image_file(file: &str, pixels: Vec<f32>, width: u32, height: u32) {
    // Normalize the output and save as an image
    let max_val = pixels.iter().cloned().fold(0.0 / 0.0, f32::max); // Find max value
    let output_image: GrayImage = ImageBuffer::from_fn(width, height, |x, y| {
        let pixel_value = (pixels[(y * width + x) as usize] / max_val * 255.0) as u8;
        Luma([pixel_value])
    });

    output_image
        .save(file)
        .expect("Failed to save output image");
    println!("Sobel edge detection complete. Output saved to {}.", file);
}

fn main() {
    let files = list_input_output_image_files();
    for (input, output) in &files {
        let (input_pixels, (width, height)) = read_image_file(input);
        let sobel_processor = ImageProcessor::new(&input_pixels, width, height);
        let output_pixels = sobel_processor.process(SobelFilter::get_kernel());
        write_image_file(&output, output_pixels, width, height);
    }
}
