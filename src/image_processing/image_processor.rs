use ocl::ProQue;

pub struct ImageProcessor<'a> {
    pixels: &'a [f32],
    dimensions: (u32, u32),
}

impl<'a> ImageProcessor<'a> {
    pub fn new(pixels: &'a [f32], dimensions: (u32, u32)) -> ImageProcessor<'a> {
        Self { pixels, dimensions }
    }

    pub fn process(&self, filter: (&str, &str)) -> Vec<f32> {
        let pro_que = ProQue::builder()
            .src(filter.0)
            .dims(self.dimensions)
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
            .kernel_builder(filter.1)
            .arg(&input_buffer)
            .arg(&output_buffer)
            .arg(&(self.dimensions.0 as i32))
            .arg(&(self.dimensions.1 as i32))
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
