use super::filters::ImageFilter;
use super::image_converter::ImageConverter;

pub struct Preprocessor;

impl Preprocessor {
    pub fn prepare(
        input: &[u32],
        filter: &Box<dyn ImageFilter>,
    ) -> ((Vec<f32>, Vec<f32>, Vec<f32>), Vec<f32>) {
        let grayscale = ImageConverter::convert_rgb_to_grayscale(input);
        let options = filter.compute_options(&grayscale);
        let kernel = filter.get_kernel();
        if kernel.1 == "gaussianBlur" || kernel.1 == "laplacianSharpening" {
            let channels = ImageConverter::decompose_rgb(input);
            (channels, options)
        } else {
            ((grayscale, vec![], vec![]), options)
        }
    }
}
