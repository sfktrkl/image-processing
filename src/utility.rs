use image::open;
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

    pub fn image_file_to_rgb(file: &str) -> (Vec<u32>, (u32, u32)) {
        let img = open(file).expect("Failed to open image");

        let image = img.to_rgb8();
        let pixels: Vec<u32> = image
            .pixels()
            .map(|p| {
                let r = p[0] as u32;
                let g = p[1] as u32;
                let b = p[2] as u32;
                let a = 255;

                (a << 24) | (r << 16) | (g << 8) | b
            })
            .collect();

        (pixels, image.dimensions())
    }

    pub fn convert_rgb_to_grayscale(pixels: &[u32]) -> Vec<f32> {
        pixels
            .chunks(1)
            .map(|pixel| {
                let r = ((pixel[0] >> 16) & 0xFF) as f32 / 255.0;
                let g = ((pixel[0] >> 8) & 0xFF) as f32 / 255.0;
                let b = (pixel[0] & 0xFF) as f32 / 255.0;

                0.2989 * r + 0.5870 * g + 0.1140 * b
            })
            .collect()
    }

    pub fn convert_grayscale_to_rgb(pixels: &[f32]) -> Vec<u32> {
        pixels
            .iter()
            .map(|&value| {
                let intensity = (value * 255.0).clamp(0.0, 255.0) as u8;

                (intensity as u32) << 16 | (intensity as u32) << 8 | (intensity as u32)
            })
            .collect()
    }

    pub fn decompose_rgb(input: &[u32]) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
        let mut r_channel = Vec::with_capacity(input.len());
        let mut g_channel = Vec::with_capacity(input.len());
        let mut b_channel = Vec::with_capacity(input.len());

        for &pixel in input {
            let r = ((pixel >> 16) & 0xFF) as f32 / 255.0;
            let g = ((pixel >> 8) & 0xFF) as f32 / 255.0;
            let b = (pixel & 0xFF) as f32 / 255.0;
            r_channel.push(r);
            g_channel.push(g);
            b_channel.push(b);
        }

        (r_channel, g_channel, b_channel)
    }

    pub fn recompose_rgb(r_channel: &[f32], g_channel: &[f32], b_channel: &[f32]) -> Vec<u32> {
        let mut output = Vec::with_capacity(r_channel.len());

        for i in 0..r_channel.len() {
            let r = (r_channel[i].min(1.0).max(0.0) * 255.0) as u32;
            let g = (g_channel[i].min(1.0).max(0.0) * 255.0) as u32;
            let b = (b_channel[i].min(1.0).max(0.0) * 255.0) as u32;
            let a = 255; // Fully opaque

            output.push((a << 24) | (r << 16) | (g << 8) | b);
        }

        output
    }
}
