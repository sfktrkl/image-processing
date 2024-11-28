pub struct SobelFilter;

pub trait ImageFilter {
    fn get_kernel() -> &'static str;
}

impl ImageFilter for SobelFilter {
    fn get_kernel() -> &'static str {
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
}
