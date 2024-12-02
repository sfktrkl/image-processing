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
}
