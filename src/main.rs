use image::{open, GrayImage, ImageBuffer, Luma};
use ocl::ProQue;

fn main() {
    // Load the input image and convert to grayscale
    let img = open("input/input.jpg")
        .expect("Failed to open image")
        .to_luma8();
    let (width, height) = img.dimensions();
    let pixels: Vec<f32> = img.pixels().map(|p| p[0] as f32).collect();

    // Define OpenCL Sobel kernel
    let src = r#"
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
    "#;

    // Create OpenCL program and queue
    let pro_que = ProQue::builder()
        .src(src)
        .dims((width, height)) // Workgroup dimensions
        .build()
        .expect("Failed to build OpenCL program");

    // Create buffers for input and output
    let input_buffer = pro_que
        .create_buffer::<f32>()
        .expect("Failed to create input buffer");
    let output_buffer = pro_que
        .create_buffer::<f32>()
        .expect("Failed to create output buffer");

    // Write input image data to GPU
    input_buffer
        .write(&pixels)
        .enq()
        .expect("Failed to write to buffer");

    // Create and execute the Sobel kernel
    let kernel = pro_que
        .kernel_builder("sobelEdgeDetection")
        .arg(&input_buffer)
        .arg(&output_buffer)
        .arg(&(width as i32))
        .arg(&(height as i32))
        .build()
        .expect("Failed to create kernel");

    unsafe {
        kernel.enq().expect("Failed to enqueue kernel");
    }

    // Read the output back to the host
    let mut output_pixels = vec![0.0f32; pixels.len()];
    output_buffer
        .read(&mut output_pixels)
        .enq()
        .expect("Failed to read buffer");

    // Normalize the output and save as an image
    let max_val = output_pixels.iter().cloned().fold(0.0 / 0.0, f32::max); // Find max value
    let output_image: GrayImage = ImageBuffer::from_fn(width, height, |x, y| {
        let pixel_value = (output_pixels[(y * width + x) as usize] / max_val * 255.0) as u8;
        Luma([pixel_value])
    });

    output_image
        .save("output/output.jpg")
        .expect("Failed to save output image");
    println!("Sobel edge detection complete. Output saved to 'output.jpg'.");
}
