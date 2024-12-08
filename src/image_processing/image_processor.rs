use super::filters::ImageFilter;
use super::image_converter::ImageConverter;
use super::opencl_processor::OpenCLProcessor;

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

    pub fn preprocess_image(
        input: &[u32],
        filter: &Box<dyn ImageFilter>,
    ) -> ((Vec<f32>, Vec<f32>, Vec<f32>), Vec<f32>) {
        let grayscale = ImageConverter::convert_rgb_to_grayscale(input);
        let options = filter.compute_options(&grayscale);
        let kernel = filter.get_kernel();
        match kernel.1 {
            "gaussianBlur" | "laplacianSharpening" | "bayerOrderedDithering" => {
                let channels = ImageConverter::decompose_rgb(input);
                (channels, options)
            }
            _ => ((grayscale, vec![], vec![]), options),
        }
    }

    pub fn process_image(&self) -> Vec<Vec<u32>> {
        self.filters
            .iter()
            .map(|filter| {
                let inputs = Self::preprocess_image(self.input, filter);
                let kernel = filter.get_kernel();
                let (channels, options) = inputs;

                let channels: Vec<Vec<f32>> = match kernel.1 {
                    "gaussianBlur" | "laplacianSharpening" | "bayerOrderedDithering" => {
                        vec![
                            OpenCLProcessor::new(&channels.0, &options, self.dimensions)
                                .process(kernel),
                            OpenCLProcessor::new(&channels.1, &options, self.dimensions)
                                .process(kernel),
                            OpenCLProcessor::new(&channels.2, &options, self.dimensions)
                                .process(kernel),
                        ]
                    }
                    _ => {
                        vec![OpenCLProcessor::new(&channels.0, &options, self.dimensions)
                            .process(kernel)]
                    }
                };

                match kernel.1 {
                    "gaussianBlur" | "bayerOrderedDithering" => {
                        ImageConverter::recompose_rgb(&channels[0], &channels[1], &channels[2])
                    }
                    "laplacianSharpening" => ImageConverter::recompose_rgb_with_original(
                        &channels[0],
                        &channels[1],
                        &channels[2],
                        self.input,
                    ),
                    _ => ImageConverter::convert_grayscale_to_rgb(&channels[0]),
                }
            })
            .collect()
    }
}
