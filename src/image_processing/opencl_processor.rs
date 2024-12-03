use ocl::ProQue;

pub struct OpenCLProcessor<'a, 'b> {
    pixels: &'a [f32],
    options: &'b [f32],
    dimensions: (u32, u32),
}

impl<'a, 'b> OpenCLProcessor<'a, 'b> {
    pub fn new(
        pixels: &'a [f32],
        options: &'b [f32],
        dimensions: (u32, u32),
    ) -> OpenCLProcessor<'a, 'b> {
        Self {
            pixels,
            options,
            dimensions,
        }
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

        let options_buffer = pro_que
            .create_buffer::<f32>()
            .expect("Failed to create options buffer");

        input_buffer
            .write(self.pixels)
            .enq()
            .expect("Failed to write to input buffer");

        if self.options.len() > 0 {
            options_buffer
                .write(self.options)
                .enq()
                .expect("Failed to write to options buffer");
        }

        let kernel = pro_que
            .kernel_builder(filter.1)
            .arg(&input_buffer)
            .arg(&output_buffer)
            .arg(&options_buffer)
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
