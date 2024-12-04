pub struct SobelFilter;
pub struct PrewittFilter;
pub struct CannyFilter;
pub struct GaussianBlur;
pub struct LaplacianSharpening;

pub trait ImageFilter {
    fn get_kernel(&self) -> (&'static str, &'static str);
    fn compute_options(&self, _: &[f32]) -> Vec<f32> {
        vec![]
    }
}

impl ImageFilter for SobelFilter {
    fn get_kernel(&self) -> (&'static str, &'static str) {
        (
            r#"
            __kernel void sobelEdgeDetection(
                __global const float* inputImage,
                __global float* outputImage,
                __global const float* options,
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
            "#,
            "sobelEdgeDetection",
        )
    }
}

impl ImageFilter for PrewittFilter {
    fn get_kernel(&self) -> (&'static str, &'static str) {
        (
            r#"
            __kernel void prewittEdgeDetection(
                __global const float* inputImage,
                __global float* outputImage,
                __global const float* options,
                const int width,
                const int height) {
                
                int x = get_global_id(0);
                int y = get_global_id(1);
        
                if (x < 1 || y < 1 || x >= width - 1 || y >= height - 1) {
                    return; // Skip the borders
                }
        
                // Prewitt X and Y kernels
                float Gx[3][3] = {{-1, 0, 1}, {-1, 0, 1}, {-1, 0, 1}};
                float Gy[3][3] = {{-1, -1, -1}, {0, 0, 0}, {1, 1, 1}};
        
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
            "#,
            "prewittEdgeDetection",
        )
    }
}

impl ImageFilter for CannyFilter {
    fn get_kernel(&self) -> (&'static str, &'static str) {
        (
            r#"
            __kernel void cannyEdgeDetection(
                __global const float* inputImage,
                __global float* outputImage,
                __global const float* options,
                const int width,
                const int height) {
                int x = get_global_id(0);
                int y = get_global_id(1);

                if (x < 1 || y < 1 || x >= width - 1 || y >= height - 1) {
                    outputImage[y * width + x] = 0.0;
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
        
                // Apply thresholds
                float lowThreshold = options[0];
                float highThreshold = options[1];
                if (magnitude > highThreshold) {
                    outputImage[y * width + x] = 1.0; // Strong edge
                } else if (magnitude > lowThreshold) {
                    outputImage[y * width + x] = 0.5; // Weak edge
                } else {
                    outputImage[y * width + x] = 0.0; // No edge
                }
            }
            "#,
            "cannyEdgeDetection",
        )
    }

    fn compute_options(&self, pixels: &[f32]) -> Vec<f32> {
        let len = pixels.len() as f32;
        let mean = pixels.iter().sum::<f32>() / len;
        let std_dev = (pixels.iter().map(|&x| (x - mean).powi(2)).sum::<f32>() / len).sqrt();

        let low_threshold = mean - std_dev;
        let high_threshold = mean + std_dev;

        vec![low_threshold.max(0.0), high_threshold.min(1.0)]
    }
}

impl ImageFilter for GaussianBlur {
    fn get_kernel(&self) -> (&'static str, &'static str) {
        (
            r#"
            __kernel void gaussianBlur(
                __global const float* inputImage,
                __global float* outputImage,
                __global const float* options,
                const int width,
                const int height) {
                
                int x = get_global_id(0);
                int y = get_global_id(1);

                int kernelOffset = 1;
                int kernelSize = (int)options[0];
                int halfKernel = kernelSize / 2;

                float sum = 0.0;
                float weightSum = 0.0;

                for (int ky = -halfKernel; ky <= halfKernel; ky++) {
                    for (int kx = -halfKernel; kx <= halfKernel; kx++) {
                        int nx = x + kx;
                        int ny = y + ky;

                        if (nx >= 0 && ny >= 0 && nx < width && ny < height) {
                            float pixel = inputImage[ny * width + nx];
                            int kernelIndex = (ky + halfKernel) * kernelSize + (kx + halfKernel);
                            float weight = options[kernelIndex + kernelOffset];
                            sum += pixel * weight;
                            weightSum += weight;
                        }
                    }
                }

                if (weightSum > 0.0) {
                    outputImage[y * width + x] = sum / weightSum;
                } else {
                    outputImage[y * width + x] = inputImage[y * width + x];
                }
            }
            "#,
            "gaussianBlur",
        )
    }

    fn compute_options(&self, _: &[f32]) -> Vec<f32> {
        let kernel_size = 5;
        let sigma = 1.0;

        let mut kernel = vec![0.0; kernel_size * kernel_size];
        let mut sum = 0.0;

        let half = kernel_size as isize / 2;
        for y in -half..=half {
            for x in -half..=half {
                let value = (-(x * x + y * y) as f32 / (2.0 * sigma * sigma)).exp();
                kernel[((y + half) as usize) * kernel_size + (x + half) as usize] = value;
                sum += value;
            }
        }

        kernel.iter_mut().for_each(|v| *v /= sum);

        let mut options = vec![kernel_size as f32];
        options.extend(kernel);
        options
    }
}

impl ImageFilter for LaplacianSharpening {
    fn get_kernel(&self) -> (&'static str, &'static str) {
        (
            r#"
                __kernel void laplacianSharpening(
                    __global const float* inputImage,
                    __global float* outputImage,
                    __global const float* options,
                    const int width,
                    const int height) {
                    
                    int x = get_global_id(0);
                    int y = get_global_id(1);

                    if (x < 1 || y < 1 || x >= width - 1 || y >= height - 1) {
                        return; // Skip the borders
                    }

                    // Laplacian kernel
                    float laplacian[3][3] = {{0, -1, 0}, {-1, 4, -1}, {0, -1, 0}};

                    float value = 0.0;
                    for (int i = -1; i <= 1; i++) {
                        for (int j = -1; j <= 1; j++) {
                            float pixel = inputImage[(y + i) * width + (x + j)];
                            value += laplacian[i + 1][j + 1] * pixel;
                        }
                    }

                    outputImage[y * width + x] = value;
                }
            "#,
            "laplacianSharpening",
        )
    }
}
