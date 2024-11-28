use image::{open, GrayImage, ImageBuffer, Luma};
use ocl::ProQue;
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

fn get_sobel_kernel() -> &'static str {
    r#"
    __kernel void sobelEdgeDetection(
        __global const float* inputImage,
        __global float* outputImage,
        const int width,
        const int height) {
        
        int x = get_global_id(0);
        int y = get_global_id(1);

        if (x < 1 || y < 1 || x >= width - 1 || y >= height - 1) {
            return; // Skip the borders
        }

        // Sobel X and Y kernels
        float Gx[3][3] = {{-1, 0, 1}, {-2, 0, 2}, {-1, 0, 1}};
        float Gy[3][3] = {{-1, -2, -1}, {0, 0, 0}, {1, 2, 1}};

        float edgeX = 0.0;
        float edgeY = 0.0;

        for (int i = -1; i <= 1; i++) {
            for (int j = -1; j <= 1; j++) {
                float pixel = inputImage[(y + i) * width + (x + j)];
                edgeX += Gx[i + 1][j + 1] * pixel;
                edgeY += Gy[i + 1][j + 1] * pixel;
            }
        }

        // Calculate magnitude of gradient
        float magnitude = sqrt(edgeX * edgeX + edgeY * edgeY);
        outputImage[y * width + x] = magnitude;
    }
    "#
}

fn process_image(pixels: Vec<f32>, width: u32, height: u32) -> Vec<f32> {
    let pro_que = ProQue::builder()
        .src(get_sobel_kernel())
        .dims((width, height))
        .build()
        .expect("Failed to build OpenCL program");

    let input_buffer = pro_que
        .create_buffer::<f32>()
        .expect("Failed to create input buffer");

    let output_buffer = pro_que
        .create_buffer::<f32>()
        .expect("Failed to create output buffer");

    input_buffer
        .write(&pixels)
        .enq()
        .expect("Failed to write to buffer");

    let kernel = pro_que
        .kernel_builder("sobelEdgeDetection")
        .arg(&input_buffer)
        .arg(&output_buffer)
        .arg(&(width as i32))
        .arg(&(height as i32))
        .build()
        .expect("Failed to create kernel");

    unsafe {
        // Execute the Sobel kernel
        kernel.enq().expect("Failed to enqueue kernel");
    }

    let mut output_pixels = vec![0.0f32; pixels.len()];
    output_buffer
        .read(&mut output_pixels)
        .enq()
        .expect("Failed to read buffer");

    output_pixels
}

fn main() {
    let files = list_input_output_image_files();
    for (input, output) in &files {
        let (input_pixels, (width, height)) = read_image_file(input);
        let output_pixels = process_image(input_pixels, width, height);
        write_image_file(&output, output_pixels, width, height);
    }
}
