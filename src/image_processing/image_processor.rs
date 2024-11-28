use ocl::ProQue;

pub struct ImageProcessor<'a> {
    pixels: &'a [f32],
    width: u32,
    height: u32,
}

impl<'a> ImageProcessor<'a> {
    pub fn new(pixels: &'a [f32], width: u32, height: u32) -> ImageProcessor<'a> {
        Self {
            pixels,
            width,
            height,
        }
    }

    pub fn process(&self, filter: &str) -> Vec<f32> {
        let pro_que = ProQue::builder()
            .src(filter)
            .dims((self.width, self.height))
            .build()
            .expect("Failed to build OpenCL program");

        let input_buffer = pro_que
            .create_buffer::<f32>()
            .expect("Failed to create input buffer");

        let output_buffer = pro_que
            .create_buffer::<f32>()
            .expect("Failed to create output buffer");

        input_buffer
            .write(self.pixels)
            .enq()
            .expect("Failed to write to buffer");

        let kernel = pro_que
            .kernel_builder("sobelEdgeDetection")
            .arg(&input_buffer)
            .arg(&output_buffer)
            .arg(&(self.width as i32))
            .arg(&(self.height as i32))
            .build()
            .expect("Failed to create kernel");

        unsafe {
            // Execute the Sobel kernel
            kernel.enq().expect("Failed to enqueue kernel");
        }

        let mut output_pixels = vec![0.0f32; self.pixels.len()];
        output_buffer
            .read(&mut output_pixels)
            .enq()
            .expect("Failed to read buffer");

        output_pixels
    }
}
