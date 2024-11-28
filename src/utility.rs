use image::{open, GrayImage, Luma};
use std::fs;

pub struct Utility;

impl Utility {
    pub fn list_input_output_image_files() -> Vec<(String, String)> {
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

    pub fn read_image_file(file: &str) -> (Vec<f32>, (u32, u32)) {
        let img = open(file).expect("Failed to open image").to_luma8();
        (
            img.pixels().map(|p| p[0] as f32).collect(),
            img.dimensions(),
        )
    }

    pub fn write_image_file(file: &str, pixels: Vec<f32>, dimensions: (u32, u32)) {
        // Normalize the output and save as an image
        let max_val = pixels.iter().cloned().fold(0.0 / 0.0, f32::max); // Find max value
        let output_image: GrayImage = GrayImage::from_fn(dimensions.0, dimensions.1, |x, y| {
            let pixel_value = (pixels[(y * dimensions.0 + x) as usize] / max_val * 255.0) as u8;
            Luma([pixel_value])
        });

        output_image
            .save(file)
            .expect("Failed to save output image");
        println!("Sobel edge detection complete. Output saved to {}.", file);
    }
}
