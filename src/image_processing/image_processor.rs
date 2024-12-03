use super::filters::ImageFilter;
use super::image_converter::ImageConverter;
use super::opencl_processor::OpenCLProcessor;
use super::preprocessor::Preprocessor;

pub struct ImageProcessor<'a, 'b> {
    input: &'a [u32],
    dimensions: (u32, u32),
    filters: &'b [Box<dyn ImageFilter>],
}

impl<'a, 'b> ImageProcessor<'a, 'b> {
    pub fn new(
        input: &'a [u32],
        dimensions: (u32, u32),
        filters: &'b [Box<dyn ImageFilter>],
    ) -> Self {
        Self {
            input,
            dimensions,
            filters,
        }
    }

    pub fn process_image(&self) -> Vec<Vec<u32>> {
        self.filters
            .iter()
            .map(|filter| {
                let inputs = Preprocessor::prepare(self.input, filter);
                let kernel = filter.get_kernel();
                let channels = inputs.0;
                let options = inputs.1;
                if kernel.1 == "gaussianBlur" {
                    let channels = vec![
                        OpenCLProcessor::new(&channels.0, &options, self.dimensions)
                            .process(kernel),
                        OpenCLProcessor::new(&channels.1, &options, self.dimensions)
                            .process(kernel),
                        OpenCLProcessor::new(&channels.2, &options, self.dimensions)
                            .process(kernel),
                    ];
                    ImageConverter::recompose_rgb(&channels[0], &channels[1], &channels[2])
                } else {
                    let channels = OpenCLProcessor::new(&channels.0, &options, self.dimensions)
                        .process(kernel);
                    ImageConverter::convert_grayscale_to_rgb(&channels)
                }
            })
            .collect()
    }
}
