pub struct SobelFilter;
pub struct PrewittFilter;
pub struct CannyFilter;

pub trait ImageFilter {
    fn get_kernel() -> (&'static str, &'static str);
}

impl ImageFilter for SobelFilter {
    fn get_kernel() -> (&'static str, &'static str) {
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
    fn get_kernel() -> (&'static str, &'static str) {
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
    fn get_kernel() -> (&'static str, &'static str) {
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
}
